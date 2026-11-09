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
        ("SHOUT", "false", False),
        # Title-cased variations of value.
        ("SHOUT", "True", True),
        ("SHOUT", "False", False),
        # Upper-cased variations of value.
        ("SHOUT", "TRUE", True),
        ("SHOUT", "FALSE", False),
        # Random-cased variations of value.
        ("SHOUT", "TruE", True),
        ("SHOUT", "truE", True),
        ("SHOUT", "FaLsE", False),
        ("SHOUT", "falsE", False),
        # Extra spaces around the value.
        ("SHOUT", "true    ", True),
        ("SHOUT", "  true  ", True),
        ("SHOUT", "    true", True),
        ("SHOUT", "false   ", False),
        ("SHOUT", "  false ", False),
        ("SHOUT", "   false", False),
        # Integer variations.
        ("SHOUT", "1", True),
        ("SHOUT", "0", False),
        # Alternative states.
        ("SHOUT", "t", True),
        ("SHOUT", "T", True),
        ("SHOUT", "  T  ", True),
        ("SHOUT", "f", False),
        ("SHOUT", "y", True),
        ("SHOUT", "n", False),
        ("SHOUT", "yes", True),
        ("SHOUT", "no", False),
        ("SHOUT", "on", True),
        ("SHOUT", "off", False),
        # Blank value variations.
        ("SHOUT", None, False),
        ("SHOUT", "", False),
        ("SHOUT", " ", False),
        ("SHOUT", "       ", False),
        # Variable names are not stripped of spaces and so don't match the
        # flag, which then naturraly takes its default value.
        ("SHOUT    ", "True", False),
        ("SHOUT    ", "False", False),
        ("  SHOUT  ", "True", False),
        ("  SHOUT  ", "False", False),
        ("    SHOUT", "True", False),
        ("    SHOUT", "False", False),
        ("SH    OUT", "True", False),
        ("SH    OUT", "False", False),
        # Same for random and reverse environment variable names: they are not
        # recognized by the option.
        ("RANDOM", "True", False),
        ("NO_SHOUT", "True", False),
        ("NO_SHOUT", "False", False),
        ("NOSHOUT", "True", False),
        ("NOSHOUT", "False", False),
    ),
)
def test_boolean_flag_envvar(runner, envvar_name, envvar_value, expected):
    assert isinstance(envvar_name, str)
    assert isinstance(envvar_value, str) or envvar_value is None

    @click.command()
    @click.option("--shout/--no-shout", envvar="SHOUT")
    def cli(shout):
        click.echo(repr(shout), nl=False)

    result = runner.invoke(cli, [], env={envvar_name: envvar_value})
    assert result.exit_code == 0
    assert result.output == repr(expected)


@pytest.mark.parametrize(
    "value",
    (
        # Extra spaces inside the value.
        "tr ue",  # codespell:ignore ue
        "fa lse",
        # Numbers.
        "10",
        "01",
        "00",
        "11",
        "0.0",
        "1.1",
        # Random strings.
        "randomstring",
        "None",
        "'None'",
        "A B",
        " 1 2 ",
        "9.3",
        "a;n",
        "x:y",
        "i/o",
    ),
)
def test_boolean_envvar_bad_values(runner, value):
    @click.command()
    @click.option("--shout/--no-shout", envvar="SHOUT")
    def cli(shout):
        click.echo(shout)

    result = runner.invoke(cli, [], env={"SHOUT": value})
    assert result.exit_code == 2
    assert (
        f"Invalid value for '--shout': {value!r} is not a valid boolean."
        in result.output
    )


def test_multiple_default_help(runner):
    @click.command()
    @click.option("--arg1", multiple=True, default=("foo", "bar"), show_default=True)
    @click.option("--arg2", multiple=True, default=(1, 2), type=int, show_default=True)
    def cmd(arg, arg2):
        pass

    result = runner.invoke(cmd, ["--help"])
    assert not result.exception
    assert "foo, bar" in result.output
    assert "1, 2" in result.output


def test_show_default_default_map(runner):
    @click.command()
    @click.option("--arg", default="a", show_default=True)
    def cmd(arg):
        click.echo(arg)

    result = runner.invoke(cmd, ["--help"], default_map={"arg": "b"})

    assert not result.exception
    assert "[default: b]" in result.output


def test_show_default_container_defaults(runner):
    """Container defaults are rendered through ``str()`` in the help output.

    Refs: https://github.com/pallets/click/issues/3036
    """

    @click.command()
    @click.option("--a", default=frozenset(["git"]), type=frozenset, show_default=True)
    @click.option("--b", default={"a": 1}, type=dict, show_default=True)
    @click.option("--c", default={"git"}, type=set, show_default=True)
    def cli(a, b, c):
        pass

    result = runner.invoke(cli, ["--help"])
    assert "[default: frozenset({'git'})]" in result.output
    assert "[default: {'a': 1}]" in result.output
    assert "[default: {'git'}]" in result.output


def test_multiple_default_type():
    opt = click.Option(["-a"], multiple=True, default=(1, 2))
    assert opt.nargs == 1
    assert opt.multiple
    assert opt.type is click.INT
    ctx = click.Context(click.Command("test"))
    assert opt.get_default(ctx) == (1, 2)


def test_multiple_default_composite_type():
    opt = click.Option(["-a"], multiple=True, default=[(1, "a")])
    assert opt.nargs == 2
    assert opt.multiple
    assert isinstance(opt.type, click.Tuple)
    assert opt.type.types == [click.INT, click.STRING]
    ctx = click.Context(click.Command("test"))
    assert opt.type_cast_value(ctx, opt.get_default(ctx)) == ((1, "a"),)


@pytest.mark.parametrize(
    "default",
    [
        set(),
        {1, 2},
        frozenset(),
        frozenset(["git"]),
        {},
        {"a": 1},
    ],
)
def test_type_from_default_container(default):
    """Container defaults have no native Click type and fall back to STRING.

    The documented "if no type is provided, the type of the default value is
    used" rule does not apply to ``set``, ``frozenset`` or ``dict`` defaults.

    Refs: https://github.com/pallets/click/issues/3036
    """
    opt = click.Option(["-a"], default=default)
    assert opt.type is click.STRING


def test_parse_multiple_default_composite_type(runner):
    @click.command()
    @click.option("-a", multiple=True, default=("a", "b"))
    @click.option("-b", multiple=True, default=[(1, "a")])
    def cmd(a, b):
        click.echo(a)
        click.echo(b)

    # result = runner.invoke(cmd, "-a c -a 1 -a d -b 2 two -b 4 four".split())
    # assert result.output == "('c', '1', 'd')\n((2, 'two'), (4, 'four'))\n"
    result = runner.invoke(cmd)
    assert result.output == "('a', 'b')\n((1, 'a'),)\n"


def test_dynamic_default_help_unset(runner):
    @click.command()
    @click.option(
        "--username",
        prompt=True,
        default=lambda: os.environ.get("USER", ""),
        show_default=True,
    )
    def cmd(username):
        print("Hello,", username)

    result = runner.invoke(cmd, ["--help"])
    assert result.exit_code == 0
    assert "--username" in result.output
    assert "lambda" not in result.output
    assert "(dynamic)" in result.output


def test_dynamic_default_help_text(runner):
    @click.command()
    @click.option(
        "--username",
        prompt=True,
        default=lambda: os.environ.get("USER", ""),
        show_default="current user",
    )
    def cmd(username):
        print("Hello,", username)

    result = runner.invoke(cmd, ["--help"])
    assert result.exit_code == 0
    assert "--username" in result.output
    assert "lambda" not in result.output
    assert "(current user)" in result.output


def test_dynamic_default_help_special_method(runner):
    class Value:
        def __call__(self):
            return 42

        def __str__(self):
            return "special value"

    opt = click.Option(["-a"], default=Value(), show_default=True)
    ctx = click.Context(click.Command("cli"))
    assert opt.get_help_extra(ctx) == {"default": "special value"}
    assert "special value" in opt.get_help_record(ctx)[1]


@pytest.mark.parametrize(
    ("type", "expect"),
    [
        (click.IntRange(1, 32), "1<=x<=32"),
        (click.IntRange(1, 32, min_open=True, max_open=True), "1<x<32"),
        (click.IntRange(1), "x>=1"),
        (click.IntRange(max=32), "x<=32"),
    ],
)
def test_intrange_default_help_text(type, expect):
    option = click.Option(["--num"], type=type, show_default=True, default=2)
    context = click.Context(click.Command("test"))
    assert option.get_help_extra(context) == {"default": "2", "range": expect}
    result = option.get_help_record(context)[1]
    assert expect in result


def test_count_default_type_help():
    """A count option with the default type should not show >=0 in help."""
    option = click.Option(["--count"], count=True, help="some words")
    context = click.Context(click.Command("test"))
    assert option.get_help_extra(context) == {}
    result = option.get_help_record(context)[1]
    assert result == "some words"


def test_file_type_help_default():
    """The default for a File type is a filename string. The string
    should be displayed in help, not an open file object.

    Type casting is only applied to defaults in processing, not when
    getting the default value.
    """
    option = click.Option(
        ["--in"], type=click.File(), default=__file__, show_default=True
    )
    context = click.Context(click.Command("test"))
    assert option.get_help_extra(context) == {"default": __file__}
    result = option.get_help_record(context)[1]
    assert __file__ in result


def test_toupper_envvar_prefix(runner):
    @click.command()
    @click.option("--arg")
    def cmd(arg):
        click.echo(arg)

    result = runner.invoke(cmd, [], auto_envvar_prefix="test", env={"TEST_ARG": "foo"})
    assert not result.exception
    assert result.output == "foo\n"


def test_nargs_envvar(runner):
    @click.command()
    @click.option("--arg", nargs=2)
    def cmd(arg):
        click.echo("|".join(arg))

    result = runner.invoke(
        cmd, [], auto_envvar_prefix="TEST", env={"TEST_ARG": "foo bar"}
    )
    assert not result.exception
    assert result.output == "foo|bar\n"

    @click.command()
    @click.option("--arg", nargs=2, multiple=True)
    def cmd(arg):
        for item in arg:
            click.echo("|".join(item))

    result = runner.invoke(
        cmd, [], auto_envvar_prefix="TEST", env={"TEST_ARG": "x 1 y 2"}
    )
    assert not result.exception
    assert result.output == "x|1\ny|2\n"


def test_show_envvar(runner):
    @click.command()
    @click.option("--arg1", envvar="ARG1", show_envvar=True)
    def cmd(arg):
        pass

    result = runner.invoke(cmd, ["--help"])
    assert not result.exception
    assert "ARG1" in result.output


def test_show_envvar_auto_prefix(runner):
    @click.command()
    @click.option("--arg1", show_envvar=True)
    def cmd(arg):
        pass

    result = runner.invoke(cmd, ["--help"], auto_envvar_prefix="TEST")
    assert not result.exception
    assert "TEST_ARG1" in result.output


def test_show_envvar_auto_prefix_dash_in_command(runner):
    @click.group()
    def cli():
        pass

    @cli.command()
    @click.option("--baz", show_envvar=True)
    def foo_bar(baz):
        pass

    result = runner.invoke(cli, ["foo-bar", "--help"], auto_envvar_prefix="TEST")
    assert not result.exception
    assert "TEST_FOO_BAR_BAZ" in result.output


def test_custom_validation(runner):
    def validate_pos_int(ctx, param, value):
        if value < 0:
            raise click.BadParameter("Value needs to be positive")
        return value

    @click.command()
    @click.option("--foo", callback=validate_pos_int, default=1)
    def cmd(foo):
        click.echo(foo)

    result = runner.invoke(cmd, ["--foo", "-1"])
    assert "Invalid value for '--foo': Value needs to be positive" in result.output

    result = runner.invoke(cmd, ["--foo", "42"])
    assert result.output == "42\n"


def test_callback_validates_prompt(runner, monkeypatch):
    def validate(ctx, param, value):
        if value < 0:
            raise click.BadParameter("should be positive")

        return value

    @click.command()
    @click.option("-a", type=int, callback=validate, prompt=True)
    def cli(a):
        click.echo(a)

    result = runner.invoke(cli, input="-12\n60\n")
    assert result.output == "A: -12\nError: should be positive\nA: 60\n60\n"


def test_winstyle_options(runner):
    @click.command()
    @click.option("/debug;/no-debug", help="Enables or disables debug mode.")
    def cmd(debug):
        click.echo(debug)

    result = runner.invoke(cmd, ["/debug"], help_option_names=["/?"])
    assert result.output == "True\n"
    result = runner.invoke(cmd, ["/no-debug"], help_option_names=["/?"])
    assert result.output == "False\n"
    result = runner.invoke(cmd, [], help_option_names=["/?"])
    assert result.output == "False\n"
    result = runner.invoke(cmd, ["/?"], help_option_names=["/?"])
    assert "/debug; /no-debug  Enables or disables debug mode." in result.output
    assert "/?                 Show this message and exit." in result.output


def test_legacy_options(runner):
    @click.command()
    @click.option("-whatever")
    def cmd(whatever):
        click.echo(whatever)

    result = runner.invoke(cmd, ["-whatever", "42"])
    assert result.output == "42\n"
    result = runner.invoke(cmd, ["-whatever=23"])
    assert result.output == "23\n"


@pytest.mark.parametrize(
    ("value", "expect_missing", "processed_value"),
    [
        (UNSET, True, None),
        (None, False, None),
        # Default type of the argument is str, so everything is processed as strings.
        ("", False, ""),
        ("  ", False, "  "),
        ("foo", False, "foo"),
        ("12", False, "12"),
        (12, False, "12"),
        (12.1, False, "12.1"),
        (list(), False, "[]"),
        (tuple(), False, "()"),
        (set(), False, "set()"),
        (frozenset(), False, "frozenset()"),
        (dict(), False, "{}"),
    ],
)
def test_required_option(value, expect_missing, processed_value):
    ctx = click.Context(click.Command(""))
    argument = click.Option(["-a"], required=True)

    if expect_missing:
        with pytest.raises(click.MissingParameter) as excinfo:
            argument.process_value(ctx, value)
        assert str(excinfo.value) == "Missing parameter: a"

    else:
        value = argument.process_value(ctx, value)
        assert value == processed_value


def test_missing_required_flag(runner):
    cli = click.Command(
        "cli", params=[click.Option(["--on/--off"], is_flag=True, required=True)]
    )
    result = runner.invoke(cli)
    assert result.exit_code == 2
    assert "Error: Missing option '--on'." in result.output


def test_missing_choice(runner):
    @click.command()
    @click.option("--foo", type=click.Choice(["foo", "bar"]), required=True)
    def cmd(foo):
        click.echo(foo)

    result = runner.invoke(cmd)
    assert result.exit_code == 2
    error, separator, choices = result.output.partition("Choose from")
    assert "Error: Missing option '--foo'. " in error
    assert "Choose from" in separator
    assert "foo" in choices
    assert "bar" in choices


def test_missing_envvar(runner):
    cli = click.Command(
        "cli", params=[click.Option(["--foo"], envvar="bar", required=True)]
    )
    result = runner.invoke(cli)
    assert result.exit_code == 2
    assert "Error: Missing option '--foo'." in result.output
    cli = click.Command(
        "cli",
        params=[click.Option(["--foo"], envvar="bar", show_envvar=True, required=True)],
    )
    result = runner.invoke(cli)
    assert result.exit_code == 2
    assert "Error: Missing option '--foo' (env var: 'bar')." in result.output

    cli = click.Command(
