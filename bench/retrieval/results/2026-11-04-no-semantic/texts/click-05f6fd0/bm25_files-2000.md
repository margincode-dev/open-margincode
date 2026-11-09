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
    )
    def cli(my_option: str):
        click.echo(f"{my_option}")

    # defaults should not give a deprecated warning
    result = runner.invoke(cli, [])
    assert result.exit_code == 0, result.output
    assert "is deprecated" not in result.output

    result = runner.invoke(cli, ["--my-option", "hello"])
    assert result.exit_code == 0, result.output
    assert "option 'my_option' is deprecated" in result.output

    if isinstance(deprecated, str):
        assert deprecated in result.output


def test_deprecated_required(runner):
    with pytest.raises(ValueError, match="is deprecated and still required"):
        click.Option(["--a"], required=True, deprecated=True)


def test_deprecated_prompt(runner):
    with pytest.raises(ValueError, match="`deprecated` options cannot use `prompt`"):
        click.Option(["--a"], prompt=True, deprecated=True)


def test_invalid_nargs(runner):
    with pytest.raises(TypeError, match="nargs=-1 is not supported for options."):

        @click.command()
        @click.option("--foo", nargs=-1)
        def cli(foo):
            pass


def test_nargs_tup_composite_mult(runner):
    @click.command()
    @click.option("--item", type=(str, int), multiple=True)
    def copy(item):
        for name, id in item:
            click.echo(f"name={name} id={id:d}")

    result = runner.invoke(copy, ["--item", "peter", "1", "--item", "max", "2"])
    assert not result.exception
    assert result.output.splitlines() == ["name=peter id=1", "name=max id=2"]


def test_counting(runner):
    @click.command()
    @click.option("-v", count=True, help="Verbosity", type=click.IntRange(0, 3))
    def cli(v):
        click.echo(f"verbosity={v:d}")

    result = runner.invoke(cli, ["-vvv"])
    assert not result.exception
    assert result.output == "verbosity=3\n"

    result = runner.invoke(cli, ["-vvvv"])
    assert result.exception
    assert "Invalid value for '-v': 4 is not in the range 0<=x<=3." in result.output

    result = runner.invoke(cli, [])
    assert not result.exception
    assert result.output == "verbosity=0\n"

    result = runner.invoke(cli, ["--help"])
    assert re.search(r"-v\s+Verbosity", result.output) is not None


@pytest.mark.parametrize("unknown_flag", ["--foo", "-f"])
def test_unknown_options(runner, unknown_flag):
    @click.command()
    def cli():
        pass

    result = runner.invoke(cli, [unknown_flag])
    assert result.exception
    assert f"No such option '{unknown_flag}'." in result.output


@pytest.mark.parametrize(
    ("value", "expect"),
    [
        ("--cat", "Did you mean '--count'?"),
        ("--bounds", "(Did you mean one of: '--bound', '--count'?)"),
        ("--bount", "(Did you mean one of: '--bound', '--count'?)"),
    ],
)
def test_suggest_possible_options(runner, value, expect):
    cli = click.Command(
        "cli", params=[click.Option(["--bound"]), click.Option(["--count"])]
    )
    result = runner.invoke(cli, [value])
    assert expect in result.output


def test_multiple_required(runner):
    @click.command()
    @click.option("-m", "--message", multiple=True, required=True)
    def cli(message):
        click.echo("\n".join(message))

    result = runner.invoke(cli, ["-m", "foo", "-mbar"])
    assert not result.exception
    assert result.output == "foo\nbar\n"

    result = runner.invoke(cli, [])
    assert result.exception
    assert "Error: Missing option '-m' / '--message'." in result.output


@pytest.mark.parametrize(
    ("multiple", "nargs", "default", "expected"),
    [
        # If multiple values are allowed, defaults should be iterable.
        (True, 1, [], ()),
        (True, 1, (), ()),
        (True, 1, tuple(), ()),
        (True, 1, set(), ()),
        (True, 1, frozenset(), ()),
        (True, 1, {}, ()),
        # Special values.
        (True, 1, None, ()),
        (True, 1, UNSET, ()),
        # Number of values are kept as-is in the default.
        (True, 1, [1], (1,)),
        (True, 1, [1, 2], (1, 2)),
        (True, 1, [1, 2, 3], (1, 2, 3)),
        (True, 1, [1.1, 2.2], (1.1, 2.2)),
        (True, 1, ["1", "2"], ("1", "2")),
        (True, 1, [None, None], (None, None)),
        # Contrary to list or tuples, native Python types not supported by Click are
        # not recognized and are converted to the default format: tuple of strings.
        # Refs: https://github.com/pallets/click/issues/3036
        (True, 1, {1, 2}, ("1", "2")),
        (True, 1, frozenset([1, 2]), ("1", "2")),
