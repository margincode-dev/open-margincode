- Argument.get_help_record (src/click/core.py:3806-3810)
```python
    def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
        if self.help is None:
            return None

        return self.make_metavar(ctx), self.help
```
- test_deprecated_usage_help_record_without_help (tests/test_arguments.py:471-481)
```python
def test_deprecated_usage_help_record_without_help(runner):
    @click.command()
    @click.argument("f", required=False, deprecated=True)
    def cli(f):
        click.echo(f)

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    # Deprecation alone produces a help row with just the deprecation label.
    assert "Positional arguments:" in result.output
    assert "(DEPRECATED)" in result.output
```
- test_deprecated_usage_help_record (tests/test_arguments.py:458-468)
```python
def test_deprecated_usage_help_record(runner, deprecated, expected_label):
    @click.command()
    @click.argument("f", required=False, deprecated=deprecated, help="path to the file")
    def cli(f):
        click.echo(f)

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    assert "Positional arguments:" in result.output
    assert "[F!]" in result.output
    assert f"path to the file {expected_label}" in result.output
```
- test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:489-495)
```python
def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
    """An argument with empty or missing help text must not gain a stray leading
    space before the deprecation label.
    """
    arg = click.Argument(["foo"], required=False, help=help_text, deprecated=deprecated)
    ctx = click.Context(click.Command("cli"))
    assert arg.get_help_record(ctx)[1] == expected
```
- Parameter.get_help_record (src/click/core.py:2850-2851)
```python
    def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
        return None
```
- test_help_spec (tests/test_options.py:85-96)
```python
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
```
- Command.format_options (src/click/core.py:1300-1310)
```python
    def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes all the options into the formatter if they exist."""
        opts = []
        for param in self.get_params(ctx):
            rv = param.get_help_record(ctx)
            if rv is not None and not isinstance(param, Argument):
                opts.append(rv)

        if opts:
            with formatter.section(_("Options")):
                formatter.write_dl(opts)
```
- Command.format_arguments (src/click/core.py:1312-1326)
```python
    def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes all arguments into the formatter, if at least one is documented.

        An argument with no help gets an empty description, the same way an option
        with no help does. That keeps the section an exhaustive list of the
        positional arguments, matching the usage line.
        """
        args = [param for param in self.get_params(ctx) if isinstance(param, Argument)]

        if any(arg.help is not None for arg in args):
            records = [
                arg.get_help_record(ctx) or (arg.make_metavar(ctx), "") for arg in args
            ]
            with formatter.section(_("Positional arguments")):
                formatter.write_dl(records)
```
- Option.get_help_record (src/click/core.py:3401-3424)
```python
    def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
        if self.hidden:
            return None

        help = self.help or ""

        extra = self.get_help_extra(ctx)
        extra_items = []
        if "envvars" in extra:
            extra_items.append(
                _("env var: {var}").format(var=", ".join(extra["envvars"]))
            )
        if "default" in extra:
            extra_items.append(_("default: {default}").format(default=extra["default"]))
        if "range" in extra:
            extra_items.append(extra["range"])
        if "required" in extra:
            extra_items.append(_(extra["required"]))

        if extra_items:
            extra_str = "; ".join(extra_items)
            help = f"{help}  [{extra_str}]" if help else f"[{extra_str}]"

        return self.get_help_spec(ctx), help
```
- test_count_default_type_help (tests/test_options.py:780-786)
```python
def test_count_default_type_help():
    """A count option with the default type should not show >=0 in help."""
    option = click.Option(["--count"], count=True, help="some words")
    context = click.Context(click.Command("test"))
    assert option.get_help_extra(context) == {}
    result = option.get_help_record(context)[1]
    assert result == "some words"
```
- test_intrange_default_help_text (tests/test_options.py:772-777)
```python
def test_intrange_default_help_text(type, expect):
    option = click.Option(["--num"], type=type, show_default=True, default=2)
    context = click.Context(click.Command("test"))
    assert option.get_help_extra(context) == {"default": "2", "range": expect}
    result = option.get_help_record(context)[1]
    assert expect in result
```
- test_deprecated_empty_help_no_leading_space (tests/test_options.py:66-72)
```python
def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
    """An option with empty or missing help text must not gain a stray leading
    space before the deprecation label.
    """
    opt = click.Option(["--foo"], help=help_text, deprecated=deprecated)
    ctx = click.Context(click.Command("cli"))
    assert opt.get_help_record(ctx)[1] == expected
```
- test_show_default_precedence (tests/test_options.py:1439-1444)
```python
def test_show_default_precedence(ctx_value, opt_value, extra_value, expect):
    ctx = click.Context(click.Command("test"), show_default=ctx_value)
    opt = click.Option("-a", default=1, help="value", show_default=opt_value)
    assert opt.get_help_extra(ctx) == extra_value
    help = opt.get_help_record(ctx)[1]
    assert ("default:" in help) is expect
```
- Argument (src/click/core.py:3706-3818)
```python
class Argument(Parameter):
    """Arguments are positional parameters to a command.  They generally
    provide fewer features than options but can have infinite ``nargs``
    and are required by default.

    All parameters are passed onwards to the constructor of :class:`Parameter`.

    :param help: the help string.

    .. versionchanged:: 8.5.0
        Added the ``help`` parameter.
    """

    param_type_name = "argument"

    def __init__(
        self,
        param_decls: cabc.Sequence[str],
        required: bool | None = None,
        help: str | None = None,
        **attrs: t.Any,
    ) -> None:
        # Auto-detect the requirement status of the argument if not explicitly set.
        if required is None:
            # The argument gets automatically required if it has no explicit default
            # value set and is setup to match at least one value.
            if attrs.get("default", UNSET) is UNSET:
                required = attrs.get("nargs", 1) > 0
            # If the argument has a default value, it is not required.
            else:
                required = False

        if "multiple" in attrs:
            raise TypeError("__init__() got an unexpected keyword argument 'multiple'.")

        deprecated = attrs.get("deprecated", False)

        if help:
            help = inspect.cleandoc(help)

        if deprecated:
            label = _format_deprecated_label(deprecated)
            help = f"{help} {label}" if help else label

        self.help = help

        super().__init__(param_decls, required=required, **attrs)

    def to_info_dict(self) -> dict[str, t.Any]:
        info_dict = super().to_info_dict()
        info_dict.update(help=self.help)
        return info_dict

    @property
    def human_readable_name(self) -> str:
        if self.metavar is not None:
            return self.metavar
        return self.name.upper()

    def make_metavar(self, ctx: Context) -> str:
        if self.metavar is not None:
            return self.metavar
        var = self.type.get_metavar(param=self, ctx=ctx)
        if not var:
            var = self.name.upper()
        # Types like ``Choice`` and ``DateTime`` already surround their metavar
        # with square brackets to enumerate the allowed values. Reuse those
        # outer brackets as the optional-argument indicator instead of wrapping
        # the metavar in a second pair, which would produce ``[[a|b|c]]``.
        already_bracketed = var.startswith("[") and var.endswith("]")
        if self.deprecated:
            var += "!"
        if not self.required and not already_bracketed:
            var = f"[{var}]"
        if self.nargs != 1:
            var += "..."
        return var

    def _parse_decls(
        self, decls: cabc.Sequence[str], expose_value: bool
    ) -> tuple[str, list[str], list[str]]:
        if not decls:
            if not expose_value:
                return "", [], []
            raise TypeError("Argument is marked as exposed, but does not have a name.")
        if len(decls) == 1:
            name = arg = decls[0]
            name = name.replace("-", "_").lower()
        else:
            raise TypeError(
                _(
                    "Arguments take exactly one parameter declaration, got"
                    " {length}: {decls}."
                ).format(length=len(decls), decls=decls)
            )
        return name, [arg], []

    def get_usage_pieces(self, ctx: Context) -> list[str]:
        return [self.make_metavar(ctx)]

    def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
        if self.help is None:
            return None

        return self.make_metavar(ctx), self.help

    def get_error_hint(self, ctx: Context | None) -> str:
        if ctx is not None:
            return f"'{self.make_metavar(ctx)}'"
        return f"'{self.human_readable_name}'"

    def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
        parser.add_argument(dest=self.name, nargs=self.nargs, obj=self)
```
- test_argument_to_info_dict_help (tests/test_info_dict.py:292-294)
```python
def test_argument_to_info_dict_help(help_in, help_out):
    arg = click.Argument(["name"], help=help_in)
    assert arg.to_info_dict()["help"] == help_out
```
- Option.get_help_spec (src/click/core.py:3370-3399)
```python
    def get_help_spec(self, ctx: Context) -> str:
        """Returns the left column of the option's help record: its spellings
        and metavar, like ``-v, --verbose`` or ``-c, --config TEXT``.

        Unlike :meth:`get_help_record`, the spec is produced even when the
        option is :attr:`hidden`.

        .. versionadded:: 8.5.1
        """
        any_prefix_is_slash = False

        def _write_opts(opts: cabc.Sequence[str]) -> str:
            nonlocal any_prefix_is_slash

            rv, any_slashes = join_options(opts)

            if any_slashes:
                any_prefix_is_slash = True

            if not self.is_flag and not self.count:
                rv += f" {self.make_metavar(ctx=ctx)}"

            return rv

        rv = [_write_opts(self.opts)]

        if self.secondary_opts:
            rv.append(_write_opts(self.secondary_opts))

        return ("; " if any_prefix_is_slash else " / ").join(rv)
```
- test_dynamic_default_help_special_method (tests/test_options.py:749-760)
```python
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
```
- test_option_custom_class (tests/test_options.py:1108-1121)
```python
def test_option_custom_class(runner):
    class CustomOption(click.Option):
        def get_help_record(self, ctx):
            """a dumb override of a help text for testing"""
            return ("--help", "I am a help text")

    @click.command()
    @click.option("--testoption", cls=CustomOption, help="you won't see me")
    def cmd(testoption):
        click.echo(testoption)

    result = runner.invoke(cmd, ["--help"])
    assert "I am a help text" in result.output
    assert "you won't see me" not in result.output
```
- test_do_not_show_default_empty_multiple (tests/test_options.py:1413-1421)
```python
def test_do_not_show_default_empty_multiple():
    """When show_default is True and multiple=True is set, it should not
    print empty default value in --help output.
    """
    opt = click.Option(["-a"], multiple=True, help="values", show_default=True)
    ctx = click.Context(click.Command("cli"))
    assert opt.get_help_extra(ctx) == {}
    message = opt.get_help_record(ctx)[1]
    assert message == "values"
```
- test_argument_help_deprecated_without_help_lists_all (tests/test_arguments.py:363-380)
```python
def test_argument_help_deprecated_without_help_lists_all(runner):
    """A deprecation label counts as help and brings every argument into view."""

    @click.command()
    @click.argument("src", required=False, deprecated=True)
    @click.argument("dst")
    def cli(src, dst):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    lines = result.output.splitlines()
    start = lines.index("Positional arguments:")
    assert lines[start : start + 3] == [
        "Positional arguments:",
        "  [SRC!]  (DEPRECATED)",
        "  DST",
    ]
```
- test_show_default_string (tests/test_options.py:1350-1356)
```python
def test_show_default_string(runner):
    """When show_default is a string show that value as default."""
    opt = click.Option(["--limit"], show_default="unlimited")
    ctx = click.Context(click.Command("cli"))
    assert opt.get_help_extra(ctx) == {"default": "(unlimited)"}
    message = opt.get_help_record(ctx)[1]
    assert "[default: (unlimited)]" in message
```
- test_argument_help_lists_undocumented_arguments (tests/test_arguments.py:327-346)
```python
def test_argument_help_lists_undocumented_arguments(runner):
    """One documented argument makes the section list all of them."""

    @click.command()
    @click.argument("src", help="Source path")
    @click.argument("dst")
    @click.argument("extra", nargs=-1)
    def cli(src, dst, extra):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    lines = result.output.splitlines()
    start = lines.index("Positional arguments:")
    assert lines[start : start + 4] == [
        "Positional arguments:",
        "  SRC         Source path",
        "  DST",
        "  [EXTRA]...",
    ]
```
- test_argument_help_empty_string_lists_all (tests/test_arguments.py:383-402)
```python
def test_argument_help_empty_string_lists_all(runner):
    """An explicit empty help opens the section, the same way an empty option help
    keeps its option listed.
    """

    @click.command()
    @click.argument("src", help="")
    @click.argument("dst")
    def cli(src, dst):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    lines = result.output.splitlines()
    start = lines.index("Positional arguments:")
    assert lines[start : start + 3] == [
        "Positional arguments:",
        "  SRC",
        "  DST",
    ]
```
- test_argument_help_undocumented_arguments_only_no_section (tests/test_arguments.py:349-360)
```python
def test_argument_help_undocumented_arguments_only_no_section(runner):
    """Arguments alone produce no section: at least one needs a help."""

    @click.command()
    @click.argument("src")
    @click.argument("dst")
    def cli(src, dst):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    assert "Positional arguments:" not in result.output
```
- test_file_type_help_default (tests/test_options.py:789-802)
```python
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
```
- test_do_not_show_no_default (tests/test_options.py:1404-1410)
```python
def test_do_not_show_no_default(runner):
    """When show_default is True and no default is set do not show None."""
    opt = click.Option(["--limit"], show_default=True)
    ctx = click.Context(click.Command("cli"))
    assert opt.get_help_extra(ctx) == {}
    message = opt.get_help_record(ctx)[1]
    assert "[default: None]" not in message
```
- test_argument_help_optional_metavar (tests/test_arguments.py:405-414)
```python
def test_argument_help_optional_metavar(runner):
    @click.command()
    @click.argument("name", required=False, default="", help="The name to print")
    def cli(name):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    assert "[NAME]" in result.output
    assert "The name to print" in result.output
```
- test_hide_false_default_boolean_flag_value (tests/test_options.py:1333-1347)
```python
def test_hide_false_default_boolean_flag_value(runner, default):
    """When a boolean flag only has one opt and its default is False or
    None, it will not show the default
    """
    opt = click.Option(
        ("--cache",),
        is_flag=True,
        show_default=True,
        default=default,
        help="Enable the cache.",
    )
    ctx = click.Context(click.Command("test"))
    assert opt.get_help_extra(ctx) == {}
    message = opt.get_help_record(ctx)[1]
    assert "[default: " not in message
```
- test_show_true_default_boolean_flag_value (tests/test_options.py:1315-1329)
```python
def test_show_true_default_boolean_flag_value(runner):
    """When a boolean flag only has one opt and its default is True,
    it will show the default value, not the opt name.
    """
    opt = click.Option(
        ("--cache",),
        is_flag=True,
        show_default=True,
        default=True,
        help="Enable the cache.",
    )
    ctx = click.Context(click.Command("test"))
    assert opt.get_help_extra(ctx) == {"default": "True"}
    message = opt.get_help_record(ctx)[1]
    assert "[default: True]" in message
```
- test_argument_to_info_dict_nargs (tests/test_info_dict.py:297-301)
```python
def test_argument_to_info_dict_nargs():
    arg = click.Argument(["files"], nargs=-1, help="files to process")
    info = arg.to_info_dict()
    assert info["nargs"] == -1
    assert info["help"] == "files to process"
```
- test_show_default_with_empty_string (tests/test_options.py:1392-1401)
```python
def test_show_default_with_empty_string(runner, default, expected):
    """The empty-string check in help rendering must not break on objects
    whose ``__eq__`` raises for string operands.

    Regression test for https://github.com/pallets/click/issues/3298.
    """
    opt = click.Option(["--limit"], default=default, show_default=True)
    ctx = click.Context(click.Command("cli"))
    message = opt.get_help_record(ctx)[1]
    assert expected in message
```
- test_argument_help (tests/test_arguments.py:295-311)
```python
def test_argument_help(runner):
    @click.command()
    @click.argument("name", help="The name to print")
    @click.option("--count", default=1, help="number of greetings")
    def cli(name, count):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    assert "Positional arguments:" in result.output
    assert "NAME" in result.output
    assert "The name to print" in result.output
    assert "Options:" in result.output
    assert "number of greetings" in result.output
    assert result.output.index("Positional arguments:") < result.output.index(
        "Options:"
    )
```
- test_option_custom_class_reusable (tests/test_options.py:1160-1185)
```python
def test_option_custom_class_reusable(runner):
    """Ensure we can reuse a custom class option. See Issue #926"""

    class CustomOption(click.Option):
        def get_help_record(self, ctx):
            """a dumb override of a help text for testing"""
            return ("--help", "I am a help text")

    # Assign to a variable to reuse the decorator.
    testoption = click.option("--testoption", cls=CustomOption, help="you won't see me")

    @click.command()
    @testoption
    def cmd1(testoption):
        click.echo(testoption)

    @click.command()
    @testoption
    def cmd2(testoption):
        click.echo(testoption)

    # Both of the commands should have the --help option now.
    for cmd in (cmd1, cmd2):
        result = runner.invoke(cmd, ["--help"])
        assert "I am a help text" in result.output
        assert "you won't see me" not in result.output
```
- test_show_default_boolean_flag_name (tests/test_options.py:1298-1312)
```python
def test_show_default_boolean_flag_name(runner, default, expect):
    """When a boolean flag has distinct True/False opts, it should show
    the default opt name instead of the default value. It should only
    show one name even if multiple are declared.
    """
    opt = click.Option(
        ("--cache/--no-cache", "--c/--nc"),
        default=default,
        show_default=True,
        help="Enable/Disable the cache.",
    )
    ctx = click.Context(click.Command("test"))
    assert opt.get_help_extra(ctx) == {"default": expect}
    message = opt.get_help_record(ctx)[1]
    assert f"[default: {expect}]" in message
```
- test_formatting_usage_custom_help (tests/test_formatting.py:236-249)
```python
def test_formatting_usage_custom_help(runner):
    @click.command(context_settings=dict(help_option_names=["--man"]))
    @click.argument("arg")
    def cmd(arg):
        click.echo(f"arg:{arg}")

    result = runner.invoke(cmd, [])
    assert result.exit_code == 2
    assert result.output.splitlines() == [
        "Usage: cmd [OPTIONS] ARG",
        "Try 'cmd --man' for help.",
        "",
        "Error: Missing argument 'ARG'.",
    ]
```
- test_command_to_info_dict_multiple_arguments (tests/test_info_dict.py:304-315)
```python
def test_command_to_info_dict_multiple_arguments():
    @click.command()
    @click.argument("src", help="source path")
    @click.argument("dst", help="destination path")
    def cli(src, dst):
        pass

    ctx = click.Context(cli)
    params = cli.to_info_dict(ctx)["params"]
    args = [p for p in params if p["param_type_name"] == "argument"]
    assert [p["name"] for p in args] == ["src", "dst"]
    assert [p["help"] for p in args] == ["source path", "destination path"]
```
- test_formatting_usage_error_no_help (tests/test_formatting.py:221-233)
```python
def test_formatting_usage_error_no_help(runner):
    @click.command(add_help_option=False)
    @click.argument("arg")
    def cmd(arg):
        click.echo(f"arg:{arg}")

    result = runner.invoke(cmd, [])
    assert result.exit_code == 2
    assert result.output.splitlines() == [
        "Usage: cmd [OPTIONS] ARG",
        "",
        "Error: Missing argument 'ARG'.",
    ]
```
- test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
```python
def test_argument_help_options_only_no_arguments_section(runner):
    @click.command()
    @click.option("--count", default=1, help="number of greetings")
    def cli(count):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0, result.output
    assert "Positional arguments:" not in result.output
    assert "Options:" in result.output
    assert "number of greetings" in result.output
```
- NAME_ARGUMENT (tests/test_info_dict.py:30-45)
```python
NAME_ARGUMENT = (
    click.Argument(["name"]),
    {
        "name": "name",
        "param_type_name": "argument",
        "opts": ["name"],
        "secondary_opts": [],
        "type": STRING_PARAM_TYPE[1],
        "required": True,
        "nargs": 1,
        "multiple": False,
        "default": None,
        "envvar": None,
        "help": None,
    },
)
```
- test_wrapping_long_command_name (tests/test_formatting.py:90-122)
```python
def test_wrapping_long_command_name(runner):
    @click.group()
    def cli():
        """Top level command"""

    @cli.group()
    def a_very_very_very_long():
        """Second level"""

    @a_very_very_very_long.command()
    @click.argument("first")
    @click.argument("second")
    @click.argument("third")
    @click.argument("fourth")
    @click.argument("fifth")
    @click.argument("sixth")
    def command():
        """A command."""

    result = runner.invoke(
        cli, ["a-very-very-very-long", "command", "--help"], terminal_width=54
    )
    assert not result.exception
    assert result.output.splitlines() == [
        "Usage: cli a-very-very-very-long command ",
        "           [OPTIONS] FIRST SECOND THIRD FOURTH FIFTH",
        "           SIXTH",
        "",
        "  A command.",
        "",
        "Options:",
        "  --help  Show this message and exit.",
    ]
```
- test_argument_default (tests/test_shell_completion.py:131-143)
```python
def test_argument_default():
    cli = Command(
        "cli",
        add_help_option=False,
        params=[
            Argument(["a"], type=Choice(["a"]), default="a"),
            Argument(["b"], type=Choice(["b"]), default="b"),
        ],
    )
    assert _get_words(cli, [], "") == ["a"]
    assert _get_words(cli, ["a"], "b") == ["b"]
    # ignore type validation
    assert _get_words(cli, ["x"], "b") == ["b"]
```
- FishComplete.format_completion (src/click/shell_completion.py:504-518)
```python
    def format_completion(self, item: CompletionItem[str]) -> str:
        """
        .. versionchanged:: 8.4.2
            Escape newlines and replace tabs with spaces in the help text to
            fix completion errors with multi-line help strings.
        """
        # According to https://fishshell.com/docs/current/cmds/complete.html
        # Command substitutions found in ARGUMENTS should return a newline-
        # separated list of arguments, and each argument may optionally have a tab
        # character followed by the argument description.
        if item.help:
            help_ = item.help.replace("\n", "\\n").replace("\t", " ")
            return f"{item.type},{item.value}\t{help_}"

        return f"{item.type},{item.value}"
```
- test_formatting_usage_error (tests/test_formatting.py:148-161)
```python
def test_formatting_usage_error(runner):
    @click.command()
    @click.argument("arg")
    def cmd(arg):
        click.echo(f"arg:{arg}")

    result = runner.invoke(cmd, [])
    assert result.exit_code == 2
    assert result.output.splitlines() == [
        "Usage: cmd [OPTIONS] ARG",
        "Try 'cmd --help' for help.",
        "",
        "Error: Missing argument 'ARG'.",
    ]
```
- test_wrapping_long_options_strings (tests/test_formatting.py:55-87)
```python
def test_wrapping_long_options_strings(runner):
    @click.group()
    def cli():
        """Top level command"""

    @cli.group()
    def a_very_long():
        """Second level"""

    @a_very_long.command()
    @click.argument("first")
    @click.argument("second")
    @click.argument("third")
    @click.argument("fourth")
    @click.argument("fifth")
    @click.argument("sixth")
    def command():
        """A command."""

    # 54 is chosen as a length where the second line is one character
    # longer than the maximum length.
    result = runner.invoke(cli, ["a-very-long", "command", "--help"], terminal_width=54)
    assert not result.exception
    assert result.output.splitlines() == [
        "Usage: cli a-very-long command [OPTIONS] FIRST SECOND",
        "                               THIRD FOURTH FIFTH",
        "                               SIXTH",
        "",
        "  A command.",
        "",
        "Options:",
        "  --help  Show this message and exit.",
    ]
```
- test_nested_subcommand_help (tests/test_arguments.py:722-741)
```python
def test_nested_subcommand_help(runner):
    @click.group()
    @click.argument("arg1")
    @click.option("--opt1")
    def cli(arg1, opt1):
        pass

    @cli.group()
    @click.argument("arg2")
    @click.option("--opt2")
    def cmd(arg2, opt2):
        pass

    @cmd.command()
    def subcmd():
        click.echo("subcommand")

    result = runner.invoke(cli, ["arg1", "cmd", "arg2", "subcmd", "--help"])
    assert not result.exception
    assert "Usage: cli ARG1 cmd ARG2 subcmd [OPTIONS]" in result.output
```
- test_subcommand_help (tests/test_arguments.py:703-719)
```python
def test_subcommand_help(runner):
    @click.group()
    @click.argument("name")
    @click.argument("val")
    @click.option("--opt")
    @click.pass_context
    def cli(ctx, name, val, opt):
        ctx.obj = dict(name=name, val=val)

    @cli.command()
    @click.pass_obj
    def cmd(obj):
        click.echo(f"CMD for {obj['name']} with value {obj['val']}")

    result = runner.invoke(cli, ["foo", "bar", "cmd", "--help"])
    assert not result.exception
    assert "Usage: cli NAME VAL cmd [OPTIONS]" in result.output
```
- test_formatting_usage_error_metavar_missing_arg (tests/test_formatting.py:164-182)
```python
def test_formatting_usage_error_metavar_missing_arg(runner):
    """
    :author: @r-m-n
    Including attribution to #612
    """

    @click.command()
    @click.argument("arg", metavar="metavar")
    def cmd(arg):
        pass

    result = runner.invoke(cmd, [])
    assert result.exit_code == 2
    assert result.output.splitlines() == [
        "Usage: cmd [OPTIONS] metavar",
        "Try 'cmd --help' for help.",
        "",
        "Error: Missing argument 'metavar'.",
    ]
```
- test_formatting_usage_error_nested (tests/test_formatting.py:201-218)
```python
def test_formatting_usage_error_nested(runner):
    @click.group()
    def cmd():
        pass

    @cmd.command()
    @click.argument("bar")
    def foo(bar):
        click.echo(f"foo:{bar}")

    result = runner.invoke(cmd, ["foo"])
    assert result.exit_code == 2
    assert result.output.splitlines() == [
        "Usage: cmd foo [OPTIONS] BAR",
        "Try 'cmd foo --help' for help.",
        "",
        "Error: Missing argument 'BAR'.",
    ]
```
- test_argument_metavar_marks_optional (tests/test_arguments.py:441-451)
```python
def test_argument_metavar_marks_optional(runner, kwargs, expected):
    """An argument is bracketed in the usage line only when it is optional."""

    @click.command()
    @click.argument("foo", **kwargs)
    def cli(foo):
        pass

    result = runner.invoke(cli, ["--help"])
    assert result.exit_code == 0
    assert result.output.splitlines()[0] == f"Usage: cli [OPTIONS] {expected}"
```
- test_choice_argument_optional_metavar (tests/test_basic.py:651-675)
```python
def test_choice_argument_optional_metavar(runner):
    """Optional Choice arguments reuse the type's brackets instead of doubling.

    Without this the usage line for a ``Choice`` argument with ``nargs=-1`` or
    ``required=False`` rendered as ``[[a|b|c]]``: one pair from ``Choice`` to
    enumerate values, a second pair from ``Argument`` to mark it optional.
    """

    @click.command()
    @click.argument("method", type=click.Choice(["foo", "bar", "baz"]), nargs=-1)
    def cli_variadic(method):
        pass

    @click.command()
    @click.argument("method", type=click.Choice(["foo", "bar", "baz"]), required=False)
    def cli_optional(method):
        pass

    variadic = runner.invoke(cli_variadic, ["--help"]).output
    assert "Usage: cli-variadic [OPTIONS] [foo|bar|baz]...\n" in variadic
    assert "[[foo|bar|baz]]" not in variadic

    optional = runner.invoke(cli_optional, ["--help"]).output
    assert "Usage: cli-optional [OPTIONS] [foo|bar|baz]\n" in optional
    assert "[[foo|bar|baz]]" not in optional
```
- test_formatting_usage_error_help_hint (tests/test_formatting.py:285-314)
```python
def test_formatting_usage_error_help_hint(
    runner, help_names, extra_options, expected_hint
):
    """The error hint should only show non-shadowed help option names,
    picking the longest for readability.

    https://github.com/pallets/click/issues/2790
    """

