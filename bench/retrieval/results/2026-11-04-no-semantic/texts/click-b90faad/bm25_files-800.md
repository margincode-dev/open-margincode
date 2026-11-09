### tests/test_options.py
```python
import enum
import os
import re
import sys
import tempfile
from contextlib import nullcontext
from typing import Literal

if sys.version_info < (3, 11):
    enum.StrEnum = enum.Enum  # type: ignore[assignment]

import pytest

import click
from click import Option
from click import UNPROCESSED
from click._utils import UNSET
from click.testing import CliRunner


def test_prefixes(runner):
    @click.command()
    @click.option("++foo", is_flag=True, help="das foo")
    @click.option("--bar", is_flag=True, help="das bar")
    def cli(foo, bar):
        click.echo(f"foo={foo} bar={bar}")

    result = runner.invoke(cli, ["++foo", "--bar"])
    assert not result.exception
    assert result.output == "foo=True bar=True\n"

    result = runner.invoke(cli, ["--help"])
    assert re.search(r"\+\+foo\s+das foo", result.output) is not None
    assert re.search(r"--bar\s+das bar", result.output) is not None


def test_invalid_option(runner):
    with pytest.raises(TypeError, match="name was passed") as exc_info:
        click.Option(["foo"])

    message = str(exc_info.value)
    assert "name was passed (foo)" in message
    assert "declare an argument" in message
    assert "'--foo'" in message


@pytest.mark.parametrize("deprecated", [True, "USE B INSTEAD"])
def test_deprecated_usage(runner, deprecated):
    @click.command()
    @click.option("--foo", default="bar", deprecated=deprecated)
    def cmd(foo):
        click.echo(foo)

    result = runner.invoke(cmd, ["--help"])
    assert "(DEPRECATED" in result.output

    if isinstance(deprecated, str):
        assert deprecated in result.output


@pytest.mark.parametrize(
    ("deprecated", "expected"),
    [(True, "(DEPRECATED)"), ("USE B INSTEAD", "(DEPRECATED: USE B INSTEAD)")],
)
@pytest.mark.parametrize("help_text", ["", None])
def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
    """An option with empty or missing help text must not gain a stray leading
    space before the deprecation label.
    """
    opt = click.Option(["--foo"], help=help_text, deprecated=deprecated)
    ctx = click.Context(click.Command("cli"))
    assert opt.get_help_record(ctx)[1] == expected


@pytest.mark.parametrize(
    ("param_decls", "kwargs", "expected"),
    [
        (["-v", "--verbose"], {"is_flag": True}, "-v, --verbose"),
        (["--config"], {}, "--config TEXT"),
        (["--color/--no-color"], {}, "--color / --no-color"),
        (["/debug;/no-debug"], {}, "/debug; /no-debug"),
    ],
)
@pytest.mark.parametrize("hidden", [False, True])
def test_help_spec(param_decls, kwargs, hidden, expected):
    """A hidden option still produces its spec via ``get_help_spec()``, unlike
    ``get_help_record()``.
    """
    opt = click.Option(param_decls, hidden=hidden, **kwargs)
    ctx = click.Context(click.Command("cli"))
    assert opt.get_help_spec(ctx) == expected

    if hidden:
        assert opt.get_help_record(ctx) is None
    else:
        assert opt.get_help_record(ctx)[0] == expected


@pytest.mark.parametrize("deprecated", [True, "USE B INSTEAD"])
def test_deprecated_warning(runner, deprecated):
    @click.command()
    @click.option(
        "--my-option", required=False, deprecated=deprecated, default="default option"
