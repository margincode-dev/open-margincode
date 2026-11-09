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
