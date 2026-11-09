### examples/naval/naval.py
```python
import click


@click.group()
@click.version_option()
def cli():
    """Naval Fate.

    This is the docopt example adopted to Click but with some actual
    commands implemented and not just the empty parsing which really
    is not all that interesting.
    """


@cli.group()
def ship():
    """Manages ships."""


@ship.command("new")
@click.argument("name")
def ship_new(name):
    """Creates a new ship."""
    click.echo(f"Created ship {name}")


@ship.command("move")
@click.argument("ship")
@click.argument("x", type=float)
@click.argument("y", type=float)
@click.option("--speed", metavar="KN", default=10, help="Speed in knots.")
def ship_move(ship, x, y, speed):
    """Moves SHIP to the new location X,Y."""
    click.echo(f"Moving ship {ship} to {x},{y} with speed {speed}")


@ship.command("shoot")
@click.argument("ship")
@click.argument("x", type=float)
@click.argument("y", type=float)
def ship_shoot(ship, x, y):
    """Makes SHIP fire to X,Y."""
    click.echo(f"Ship {ship} fires to {x},{y}")


@cli.group("mine")
def mine():
    """Manages mines."""


@mine.command("set")
@click.argument("x", type=float)
@click.argument("y", type=float)
@click.option(
    "ty",
    "--moored",
    flag_value="moored",
    default=True,
    help="Moored (anchored) mine. Default.",
)
@click.option("ty", "--drifting", flag_value="drifting", help="Drifting mine.")
def mine_set(x, y, ty):
    """Sets a mine at a specific coordinate."""
    click.echo(f"Set {ty} mine at {x},{y}")


@mine.command("remove")
@click.argument("x", type=float)
@click.argument("y", type=float)
def mine_remove(x, y):
    """Removes a mine at a specific coordinate."""
    click.echo(f"Removed mine at {x},{y}")
```
### tests/test_commands.py
```python
import re

import pytest

import click


def test_other_command_invoke(runner):
    @click.command()
    @click.pass_context
    def cli(ctx):
        return ctx.invoke(other_cmd, arg=42)

    @click.command()
    @click.argument("arg", type=click.INT)
    def other_cmd(arg):
        click.echo(arg)

    result = runner.invoke(cli, [])
    assert not result.exception
    assert result.output == "42\n"


def test_other_command_forward(runner):
    cli = click.Group()

    @cli.command()
    @click.option("--count", default=1)
    def test(count):
        click.echo(f"Count: {count:d}")

    @cli.command()
    @click.option("--count", default=1)
    @click.pass_context
    def dist(ctx, count):
        ctx.forward(test)
        ctx.invoke(test, count=42)

    result = runner.invoke(cli, ["dist"])
    assert not result.exception
    assert result.output == "Count: 1\nCount: 42\n"


def test_forwarded_params_consistency(runner):
    cli = click.Group()

    @cli.command()
    @click.option("-a")
    @click.pass_context
    def first(ctx, **kwargs):
        click.echo(f"{ctx.params}")

    @cli.command()
    @click.option("-a")
    @click.option("-b")
    @click.pass_context
    def second(ctx, **kwargs):
        click.echo(f"{ctx.params}")
        ctx.forward(first)

    result = runner.invoke(cli, ["second", "-a", "foo", "-b", "bar"])
    assert not result.exception
    assert result.output == "{'a': 'foo', 'b': 'bar'}\n{'a': 'foo', 'b': 'bar'}\n"


def test_auto_shorthelp(runner):
    @click.group()
    def cli():
        pass

    @cli.command()
    def short():
        """This is a short text."""

    @cli.command()
    def special_chars():
        """Login and store the token in ~/.netrc."""

    @cli.command()
    def long():
        """This is a long text that is too long to show as short help
        and will be truncated instead."""

    result = runner.invoke(cli, ["--help"])
    assert (
        re.search(
            r"Commands:\n\s+"
            r"long\s+This is a long text that is too long to show as short help"
            r"\.\.\.\n\s+"
            r"short\s+This is a short text\.\n\s+"
            r"special-chars\s+Login and store the token in ~/.netrc\.\s*",
            result.output,
        )
        is not None
    )


def test_command_no_args_is_help(runner):
    result = runner.invoke(click.Command("test", no_args_is_help=True))
    assert result.exit_code == 2
    assert "Show this message and exit." in result.output


def test_default_maps(runner):
    @click.group()
    def cli():
        pass

    @cli.command()
    @click.option("--name", default="normal")
    def foo(name):
        click.echo(name)

    result = runner.invoke(cli, ["foo"], default_map={"foo": {"name": "changed"}})

    assert not result.exception
    assert result.output == "changed\n"


@pytest.mark.parametrize(
    ("args", "exit_code", "expect"),
    [
        (["obj1"], 2, "Error: Missing command."),
        (["obj1", "--help"], 0, "Show this message and exit."),
        (["obj1", "move"], 0, "obj=obj1\nmove\n"),
        ([], 2, "Show this message and exit."),
    ],
)
def test_group_with_args(runner, args, exit_code, expect):
    @click.group()
    @click.argument("obj")
    def cli(obj):
        click.echo(f"obj={obj}")

    @cli.command()
    def move():
        click.echo("move")

    result = runner.invoke(cli, args)
    assert result.exit_code == exit_code
    assert expect in result.output


def test_custom_parser(runner):
    import optparse

    @click.group()
    def cli():
        pass

    class OptParseCommand(click.Command):
        def __init__(self, name, parser, callback):
            super().__init__(name)
            self.parser = parser
            self.callback = callback

        def parse_args(self, ctx, args):
            try:
                opts, args = parser.parse_args(args)
            except Exception as e:
                ctx.fail(str(e))
            ctx.args = args
            ctx.params = vars(opts)

        def get_usage(self, ctx):
            return self.parser.get_usage()

        def get_help(self, ctx):
            return self.parser.format_help()

        def invoke(self, ctx):
            ctx.invoke(self.callback, ctx.args, **ctx.params)

    parser = optparse.OptionParser(usage="Usage: foo test [OPTIONS]")
    parser.add_option(
        "-f", "--file", dest="filename", help="write report to FILE", metavar="FILE"
    )
    parser.add_option(
        "-q",
        "--quiet",
        action="store_false",
        dest="verbose",
        default=True,
        help="don't print status messages to stdout",
    )

    def test_callback(args, filename, verbose):
        click.echo(" ".join(args))
        click.echo(filename)
        click.echo(verbose)

    cli.add_command(OptParseCommand("test", parser, test_callback))

    result = runner.invoke(cli, ["test", "-f", "f.txt", "-q", "q1.txt", "q2.txt"])
    assert result.exception is None
    assert result.output.splitlines() == ["q1.txt q2.txt", "f.txt", "False"]

    result = runner.invoke(cli, ["test", "--help"])
    assert result.exception is None
    assert result.output.splitlines() == [
        "Usage: foo test [OPTIONS]",
        "",
        "Options:",
        "  -h, --help            show this help message and exit",
        "  -f FILE, --file=FILE  write report to FILE",
        "  -q, --quiet           don't print status messages to stdout",
    ]


def test_object_propagation(runner):
    for chain in False, True:

        @click.group(chain=chain)
        @click.option("--debug/--no-debug", default=False)
        @click.pass_context
        def cli(ctx, debug):
            if ctx.obj is None:
                ctx.obj = {}
            ctx.obj["DEBUG"] = debug

        @cli.command()
        @click.pass_context
        def sync(ctx):
            click.echo(f"Debug is {'on' if ctx.obj['DEBUG'] else 'off'}")

        result = runner.invoke(cli, ["sync"])
        assert result.exception is None
        assert result.output == "Debug is off\n"


@pytest.mark.parametrize(
    ("opt_params", "expected"),
    (
        # Original tests.
        ({"type": click.INT, "default": 42}, 42),
        ({"type": click.INT, "default": "15"}, 15),
        ({"multiple": True}, ()),
        # SENTINEL value tests.
        ({"default": None}, None),
        ({"type": click.STRING}, None),  # No default specified, should be None.
        ({"type": click.BOOL, "default": False}, False),
        ({"type": click.BOOL, "default": True}, True),
        ({"type": click.FLOAT, "default": 3.14}, 3.14),
        # Multiple with default.
        ({"multiple": True, "default": [1, 2, 3]}, (1, 2, 3)),
        ({"multiple": True, "default": ()}, ()),
        # Required option without value should use SENTINEL behavior.
        ({"required": False}, None),
        # Choice type with default.
        ({"type": click.Choice(["a", "b", "c"]), "default": "b"}, "b"),
        # Path type with default.
        ({"type": click.Path(), "default": "/tmp"}, "/tmp"),
        # Flag options.
        ({"is_flag": True, "default": False}, False),
        ({"is_flag": True, "default": True}, True),
        # Count option.
        ({"count": True}, 0),
        # Hidden option.
        ({"hidden": True, "default": "secret"}, "secret"),
    ),
)
def test_other_command_invoke_with_defaults(runner, opt_params, expected):
    @click.command()
    @click.pass_context
    def cli(ctx):
        return ctx.invoke(other_cmd)

    @click.command()
    @click.option("-a", **opt_params)
    @click.pass_context
    def other_cmd(ctx, a):
        return ctx.info_name, a

    result = runner.invoke(cli, standalone_mode=False)

    assert result.return_value == ("other", expected)


def test_invoked_subcommand(runner):
    @click.group(invoke_without_command=True)
    @click.pass_context
    def cli(ctx):
        if ctx.invoked_subcommand is None:
            click.echo("no subcommand, use default")
            ctx.invoke(sync)
        else:
            click.echo("invoke subcommand")

    @cli.command()
    def sync():
        click.echo("in subcommand")

    result = runner.invoke(cli, ["sync"])
    assert not result.exception
    assert result.output == "invoke subcommand\nin subcommand\n"

    result = runner.invoke(cli)
    assert not result.exception
    assert result.output == "no subcommand, use default\nin subcommand\n"


@pytest.mark.parametrize(
    ("chain", "invoke_without_command", "metavar"),
    [
        (False, False, "COMMAND [ARGS]..."),
        (False, True, "[COMMAND] [ARGS]..."),
        (True, False, "COMMAND1 [ARGS]... [COMMAND2 [ARGS]...]..."),
        (True, True, "[COMMAND1] [ARGS]... [COMMAND2 [ARGS]...]..."),
    ],
)
def test_subcommand_metavar_marks_optional(
    runner, chain, invoke_without_command, metavar
):
    """The leading subcommand token is bracketed only when it is optional."""

    @click.group(chain=chain, invoke_without_command=invoke_without_command)
    def cli():
        pass

    @cli.command()
    def sub():
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0
    assert result.output.splitlines()[0] == f"Usage: cli [OPTIONS] {metavar}"


def test_aliased_command_canonical_name(runner):
    class AliasedGroup(click.Group):
        def get_command(self, ctx, cmd_name):
            return push

        def resolve_command(self, ctx, args):
            _, command, args = super().resolve_command(ctx, args)
            return command.name, command, args

    cli = AliasedGroup()

    @cli.command()
    def push():
        click.echo("push command")

    result = runner.invoke(cli, ["pu", "--help"])
    assert not result.exception
    assert result.output.startswith("Usage: root push [OPTIONS]")


def test_group_add_command_name(runner):
    cli = click.Group("cli")
    cmd = click.Command("a", params=[click.Option(["-x"], required=True)])
    cli.add_command(cmd, "b")
    # Check that the command is accessed through the registered name,
    # not the original name.
    result = runner.invoke(cli, ["b"], default_map={"b": {"x": 3}})
    assert result.exit_code == 0


@pytest.mark.parametrize(
    ("invocation_order", "declaration_order", "expected_order"),
    [
        # Non-eager options.
        ([], ["-a"], ["-a"]),
        (["-a"], ["-a"], ["-a"]),
        ([], ["-a", "-c"], ["-a", "-c"]),
        (["-a"], ["-a", "-c"], ["-a", "-c"]),
        (["-c"], ["-a", "-c"], ["-c", "-a"]),
        ([], ["-c", "-a"], ["-c", "-a"]),
        (["-a"], ["-c", "-a"], ["-a", "-c"]),
        (["-c"], ["-c", "-a"], ["-c", "-a"]),
        (["-a", "-c"], ["-a", "-c"], ["-a", "-c"]),
        (["-c", "-a"], ["-a", "-c"], ["-c", "-a"]),
        # Eager options.
        ([], ["-b"], ["-b"]),
        (["-b"], ["-b"], ["-b"]),
        ([], ["-b", "-d"], ["-b", "-d"]),
        (["-b"], ["-b", "-d"], ["-b", "-d"]),
        (["-d"], ["-b", "-d"], ["-d", "-b"]),
        ([], ["-d", "-b"], ["-d", "-b"]),
        (["-b"], ["-d", "-b"], ["-b", "-d"]),
        (["-d"], ["-d", "-b"], ["-d", "-b"]),
        (["-b", "-d"], ["-b", "-d"], ["-b", "-d"]),
        (["-d", "-b"], ["-b", "-d"], ["-d", "-b"]),
        # Mixed options.
        ([], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-a", "-c"]),
        (["-a"], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-a", "-c"]),
        (["-b"], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-a", "-c"]),
        (["-c"], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-c", "-a"]),
        (["-d"], ["-a", "-b", "-c", "-d"], ["-d", "-b", "-a", "-c"]),
        (["-a", "-b"], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-a", "-c"]),
        (["-b", "-a"], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-a", "-c"]),
        (["-d", "-c"], ["-a", "-b", "-c", "-d"], ["-d", "-b", "-c", "-a"]),
        (["-c", "-d"], ["-a", "-b", "-c", "-d"], ["-d", "-b", "-c", "-a"]),
        (["-a", "-b", "-c", "-d"], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-a", "-c"]),
        (["-b", "-d", "-a", "-c"], ["-a", "-b", "-c", "-d"], ["-b", "-d", "-a", "-c"]),
        ([], ["-b", "-d", "-e", "-a", "-c"], ["-b", "-d", "-e", "-a", "-c"]),
        (["-a", "-d"], ["-b", "-d", "-e", "-a", "-c"], ["-d", "-b", "-e", "-a", "-c"]),
        (["-c", "-d"], ["-b", "-d", "-e", "-a", "-c"], ["-d", "-b", "-e", "-c", "-a"]),
    ],
)
def test_iter_params_for_processing(
    invocation_order, declaration_order, expected_order
):
    parameters = {
        "-a": click.Option(["-a"]),
        "-b": click.Option(["-b"], is_eager=True),
        "-c": click.Option(["-c"]),
        "-d": click.Option(["-d"], is_eager=True),
        "-e": click.Option(["-e"], is_eager=True),
    }

    invocation_params = [parameters[opt_id] for opt_id in invocation_order]
    declaration_params = [parameters[opt_id] for opt_id in declaration_order]
    expected_params = [parameters[opt_id] for opt_id in expected_order]

    assert (
