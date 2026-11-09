### tests/test_utils/test_prompt.py
```python
import sys
from io import StringIO

import pytest

import click
from click._compat import WIN


def test_prompt_cast_default(capfd, monkeypatch):
    monkeypatch.setattr(sys, "stdin", StringIO("\n"))
    value = click.prompt("value", default="100", type=int)
    capfd.readouterr()
    assert isinstance(value, int)


def test_prompt_default_round_trips_through_type(capfd, monkeypatch):
    """A default that already has the converted value type still passes
    through ``ParamType.convert``, so conversion side effects keep
    applying to defaults."""

    class DoublingType(click.ParamType[int]):
        name = "doubling"

        def convert(self, value, param, ctx):
            return int(value) * 2

    monkeypatch.setattr(sys, "stdin", StringIO("\n"))
    value = click.prompt("value", default=5, type=DoublingType())
    capfd.readouterr()
    assert value == 10


def test_prompt_default_validated_by_type(capfd, monkeypatch):
    """An out-of-range default is rejected by the type's validation and
    prompts again instead of being returned as-is."""
    monkeypatch.setattr(sys, "stdin", StringIO("\n7\n"))
    value = click.prompt("value", default=100, type=click.IntRange(0, 10))
    capfd.readouterr()
    assert value == 7


@pytest.mark.skipif(WIN, reason="Different behavior on windows.")
def test_prompts_abort(monkeypatch, capsys):
    def f(_):
        raise KeyboardInterrupt()

    monkeypatch.setattr("click.termui.hidden_prompt_func", f)

    try:
        click.prompt("Password", hide_input=True)
    except click.Abort:
        click.echo("interrupted")

    out, err = capsys.readouterr()
    # On non-Windows, prompt is passed directly to getpass, not echoed separately
    assert out == "\ninterrupted\n"


@pytest.mark.skipif(WIN, reason="Different behavior on windows.")
@pytest.mark.parametrize(
    ("call", "expected_prompt"),
    [
        (lambda: click.prompt("Name"), "Name: "),
        (lambda: click.prompt("Pw", hide_input=True), "Pw: "),
        (lambda: click.prompt("IP", prompt_suffix="."), "IP."),
        (lambda: click.confirm("OK"), "OK [y/N]: "),
    ],
    ids=["prompt", "prompt-hidden", "prompt-custom-suffix", "confirm"],
)
def test_full_prompt_passed_to_readline(monkeypatch, call, expected_prompt):
    """On non-Windows, prompt and confirm pass the full prompt text to the
    underlying prompt function so readline handles editing correctly.

    https://github.com/pallets/click/issues/2968
    https://github.com/pallets/click/pull/2969
    """
    received = []

    def capture(text):
        received.append(text)
        return "y"

    monkeypatch.setattr("click.termui.visible_prompt_func", capture)
    monkeypatch.setattr("click.termui.hidden_prompt_func", capture)
    call()
    assert received == [expected_prompt]


def test_prompts_eof(runner):
    """If too few lines of input are given, prompt should exit, not hang."""

    @click.command
    def echo():
        for _ in range(3):
            click.echo(click.prompt("", type=int))

    # only provide two lines of input for three prompts
    result = runner.invoke(echo, input="1\n2\n")
    assert result.exit_code == 1
```
### src/click/termui.py
```python
from __future__ import annotations

import collections.abc as cabc
import inspect
import io
import itertools
import os
import re
import sys
import typing as t
from contextlib import AbstractContextManager
from contextlib import redirect_stdout
from gettext import gettext as _

from . import _compat
from ._compat import isatty
from ._compat import strip_ansi
from .exceptions import Abort
from .exceptions import UsageError
from .globals import resolve_color_default
from .types import Choice
from .types import convert_type
from .types import ParamType
from .utils import _LazyFile
from .utils import echo

if t.TYPE_CHECKING:
    from ._termui_impl import ProgressBar

V = t.TypeVar("V")

# The prompt functions to use.  The doc tools currently override these
# functions to customize how they work.
visible_prompt_func: t.Callable[[str], str] = input

_ansi_colors = {
    "black": 30,
    "red": 31,
    "green": 32,
    "yellow": 33,
    "blue": 34,
    "magenta": 35,
    "cyan": 36,
    "white": 37,
    "reset": 39,
    "bright_black": 90,
    "bright_red": 91,
    "bright_green": 92,
    "bright_yellow": 93,
    "bright_blue": 94,
    "bright_magenta": 95,
    "bright_cyan": 96,
    "bright_white": 97,
}
_ansi_reset_all = "\033[0m"


_HIDDEN_INPUT_MASK = "'***'"


def _mask_hidden_input(message: str, value: str) -> str:
    """Replace occurrences of ``value`` in ``message`` with a fixed mask.

    Both ``repr(value)`` (the form built-in :class:`ParamType` errors use
    via ``{value!r}``) and the raw value are masked. The raw-value pass
    uses word-boundary lookarounds so a substring like ``"1"`` does not
    match inside ``"10"``, and ``"ent"`` does not match inside
    ``"Authentication"``. The empty string is skipped to avoid matching
    at every boundary.
    """
    message = message.replace(repr(value), _HIDDEN_INPUT_MASK)
    if value:
        message = re.sub(
            rf"(?<!\w){re.escape(value)}(?!\w)", _HIDDEN_INPUT_MASK, message
        )
    return message


def hidden_prompt_func(prompt: str) -> str:
    import getpass

    return getpass.getpass(prompt)


def _readline_prompt(func: t.Callable[[str], str], text: str, err: bool) -> str:
    """Call a prompt function, passing the full prompt so readline can
    handle line editing and cursor positioning correctly.

    The prompt is handed to *func* (such as :func:`input`) rather than
    written through :func:`echo`, so it has to strip ANSI color and style
    codes itself when the destination stream does not support them. Without
    this the prompt would keep codes that :func:`echo` removes from the
    rest of the output.
    """
    stream = sys.stderr if err else sys.stdout

    # Look up ``should_strip_ansi`` on the module so that ``CliRunner``,
    # which patches it there during test isolation, is honored.
    if _compat.should_strip_ansi(stream, resolve_color_default()):
        text = strip_ansi(text)

    if err:
        with redirect_stdout(sys.stderr):
            return func(text)
    return func(text)


def _build_prompt(
    text: str,
    suffix: str,
    show_default: bool | str = False,
    default: object | None = None,
    show_choices: bool = True,
    type: object | None = None,
) -> str:
    prompt = text
    if type is not None and show_choices and isinstance(type, Choice):
        prompt += f" ({', '.join(map(str, type.choices))})"
    default_preview = ""
    if show_default:
        if isinstance(show_default, str):
            default_preview = f" [({show_default})]"
        elif default is not None:
            default_preview = f" [{_format_default(default)}]"
    return f"{prompt}{default_preview}{suffix}"


def _format_default(default: V) -> V | str:
    if isinstance(default, (io.IOBase, _LazyFile)):
        name = getattr(default, "name", None)

        if name is not None:
            return str(name)

    return default


@t.overload
def prompt(
    text: str,
    default: str | None = None,
    hide_input: bool = False,
    confirmation_prompt: bool | str = False,
    type: None = None,
    value_proc: None = None,
    prompt_suffix: str = ": ",
    show_default: bool | str = True,
    err: bool = False,
    show_choices: bool = True,
) -> str: ...


@t.overload
def prompt(
    text: str,
    default: V | str | None = None,
    hide_input: bool = False,
    confirmation_prompt: bool | str = False,
    type: ParamType[V, str] | type[V] | None = None,
    value_proc: t.Callable[[str], V] | None = None,
    prompt_suffix: str = ": ",
    show_default: bool | str = True,
    err: bool = False,
    show_choices: bool = True,
) -> V: ...


def prompt(
    text: str,
    default: V | str | None = None,
