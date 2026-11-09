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
        (True, 1, {1: "a", 2: "b"}, ("1", "2")),
        # Multiple values with nargs > 1.
        (True, 2, [], ()),
        (True, 2, (), ()),
        (True, 2, tuple(), ()),
        (True, 2, set(), ()),
        (True, 2, frozenset(), ()),
        (True, 2, {}, ()),
        (True, 2, None, ()),
        (True, 2, UNSET, ()),
        (True, 2, [[1, 2]], ((1, 2),)),
        (True, 2, [[1, 2], [3, 4]], ((1, 2), (3, 4))),
        (True, 2, [[1, 2], [3, 4], [5, 6]], ((1, 2), (3, 4), (5, 6))),
        (True, 2, [[1.1, 2.2], [3.3, 4.4]], ((1.1, 2.2), (3.3, 4.4))),
        (True, 2, [["1", "2"], ["3", "4"]], (("1", "2"), ("3", "4"))),
        (True, 2, [[None, None], [None, None]], ((None, None), (None, None))),
        (True, 2, [[1, 2.2], ["3", None]], ((1, 2.2), (3, None))),
        (True, 2, [[1, 2.2], None], ((1, 2.2), None)),
        # Default of the right length works for non-multiples.
        (False, 2, [1, 2], (1, 2)),
    ],
)
def test_good_defaults_for_multiple(runner, multiple, nargs, default, expected):
    """Comprehensive check of default-value processing for options with
    ``multiple=True`` and/or ``nargs > 1``.

    .. hint::
        An argument-specific equivalent is in
        ``test_arguments.py::test_good_defaults_for_nargs``.

        Smoke tests are in ``test_defaults.py``:
        ``test_multiple_defaults`` (explicit ``type=FLOAT``)
        and ``test_nargs_plus_multiple`` (``nargs=2``).
    """

    @click.command()
    @click.option("-a", multiple=multiple, nargs=nargs, default=default)
    def cmd(a):
        click.echo(repr(a), nl=False)

    result = runner.invoke(cmd)
    assert result.output == repr(expected)


@pytest.mark.parametrize(
    ("multiple", "nargs", "default", "exception", "message"),
    [
        # Non-iterables defaults.
        (
            True,
            1,
            "Yo",
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            1,
            "",
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            1,
            True,
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            1,
            False,
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            1,
            12,
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            1,
            7.9,
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            False,
            2,
            42,
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            2,
            ["test string which is not a list in the list"],
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        # Multiple options, each with 2 args, but with wrong length.
        (
            True,
            2,
            (1,),
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            2,
            (1, 2, 3),
            None,
            "Error: Invalid value for '-a': Value must be an iterable.",
        ),
        (
            True,
            2,
            [tuple()],
            ValueError,
            r"'nargs' must be 0 \(or None\) for type <click\.types\.Tuple object at "
            r"0x[0-9A-Fa-f]+>, but it was 2\.",
        ),
        (
            True,
            2,
            [(1,)],
            ValueError,
            r"'nargs' must be 1 \(or None\) for type <click\.types\.Tuple object at "
            r"0x[0-9A-Fa-f]+>, but it was 2\.",
        ),
        (
            True,
            2,
            [(1, 2, 3)],
            ValueError,
            r"'nargs' must be 3 \(or None\) for type <click\.types\.Tuple object at "
            r"0x[0-9A-Fa-f]+>, but it was 2\.",
        ),
        # A mix of valid and invalid defaults.
        (
            True,
            2,
            [[1, 2.2], []],
            None,
            "Error: Invalid value for '-a': 2 values are required, but 0 were given.",
        ),
        # Default values that are iterable but not of the right length.
        (
            False,
            2,
            [1],
            None,
            "Error: Invalid value for '-a': Takes 2 values but 1 was given.",
        ),
        (
            True,
            2,
            [[1]],
            ValueError,
            r"'nargs' must be 1 \(or None\) for type <click\.types\.Tuple object at "
            r"0x[0-9A-Fa-f]+>, but it was 2\.",
        ),
    ],
)
def test_bad_defaults_for_multiple(
    runner, multiple, nargs, default, exception, message
):
    if exception:
        assert issubclass(exception, Exception)
    else:
        assert exception is None

    with (
        pytest.raises(exception, match=re.compile(message))
        if exception
        else nullcontext()
    ):

        @click.command()
        @click.option("-a", multiple=multiple, nargs=nargs, default=default)
        def cmd(a):
            click.echo(repr(a))

        result = runner.invoke(cmd)
        assert message in result.stderr


@pytest.mark.parametrize("env_key", ["MYPATH", "AUTO_MYPATH"])
def test_empty_envvar(runner, env_key):
    @click.command()
    @click.option("--mypath", type=click.Path(exists=True), envvar="MYPATH")
    def cli(mypath):
        click.echo(f"mypath: {mypath}")

    result = runner.invoke(cli, env={env_key: ""}, auto_envvar_prefix="AUTO")
    assert result.exception is None
    assert result.output == "mypath: None\n"


def test_multiple_envvar(runner):
    @click.command()
    @click.option("--arg", multiple=True)
    def cmd(arg):
        click.echo("|".join(arg))

    result = runner.invoke(
        cmd, [], auto_envvar_prefix="TEST", env={"TEST_ARG": "foo bar baz"}
    )
    assert not result.exception
    assert result.output == "foo|bar|baz\n"


def test_envvar_container_type(runner):
    """Env vars are plain strings, so a container type splits them character-wise.

    Refs: https://github.com/pallets/click/issues/3036
    """

    @click.command()
    @click.option("--x", envvar="X", type=frozenset)
    def cli(x):
        assert isinstance(x, frozenset)
        click.echo("".join(sorted(x)))

    result = runner.invoke(cli, [], env={"X": "git"})
    assert not result.exception
    assert result.output == "git\n"

    @click.command()
    @click.option("--arg", multiple=True, envvar="X")
    def cmd(arg):
        click.echo("|".join(arg))

    result = runner.invoke(cmd, [], env={"X": "foo bar baz"})
    assert not result.exception
    assert result.output == "foo|bar|baz\n"

    @click.command()
    @click.option("--arg", multiple=True, type=click.Path())
    def cmd(arg):
        click.echo("|".join(arg))

    result = runner.invoke(
        cmd,
        [],
        auto_envvar_prefix="TEST",
        env={"TEST_ARG": f"foo{os.path.pathsep}bar"},
    )
    assert not result.exception
    assert result.output == "foo|bar\n"


@pytest.mark.parametrize(
    ("envvar_name", "envvar_value", "expected"),
    (
        # Lower-cased variations of value.
        ("SHOUT", "true", True),
