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
- Command.format_arguments (src/click/core.py:1312-1322)
```python
    def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes the arguments that have a help record into the formatter."""
        args = []
        for param in self.get_params(ctx):
            rv = param.get_help_record(ctx)
            if rv is not None and isinstance(param, Argument):
                args.append(rv)

        if args:
            with formatter.section(_("Positional arguments")):
                formatter.write_dl(args)
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
- argument (src/click/decorators.py:324-349)
```python
def argument(
    *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any
) -> t.Callable[[FC], FC]:
    """Attaches an argument to the command.  All positional arguments are
    passed as parameter declarations to :class:`Argument`; all keyword
    arguments are forwarded unchanged (except ``cls``).
    This is equivalent to creating an :class:`Argument` instance manually
    and attaching it to the :attr:`Command.params` list.

    For the default argument class, refer to :class:`Argument` and
    :class:`Parameter` for descriptions of parameters.

    :param cls: the argument class to instantiate.  This defaults to
                :class:`Argument`.
    :param param_decls: Passed as positional arguments to the constructor of
        ``cls``.
    :param attrs: Passed as keyword arguments to the constructor of ``cls``.
    """
    if cls is None:
        cls = Argument

    def decorator(f: FC) -> FC:
        _param_memo(f, cls(param_decls, **attrs))
        return f

    return decorator
```
- test_deprecated_usage_help_record_without_help (tests/test_arguments.py:393-403)
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
- Argument (src/click/core.py:3702-3814)
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
- test_deprecated_usage_help_record (tests/test_arguments.py:380-390)
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
- test_datetime_argument_optional_metavar (tests/test_basic.py:678-688)
```python
def test_datetime_argument_optional_metavar(runner):
    """``DateTime`` arguments behave the same way as ``Choice``."""

    @click.command()
    @click.argument("when", type=click.DateTime(formats=["%Y-%m-%d"]), required=False)
    def cli(when):
        pass

    result = runner.invoke(cli, ["--help"])
    assert "Usage: cli [OPTIONS] [%Y-%m-%d]\n" in result.output
    assert "[[%Y-%m-%d]]" not in result.output
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
- _OptionParser.add_argument (src/click/parser.py:290-296)
```python
    def add_argument(self, obj: CoreArgument, dest: str | None, nargs: int = 1) -> None:
        """Adds a positional argument named `dest` to the parser.

        The `obj` can be used to identify the option in the order list
        that is returned from the parser.
        """
        self._args.append(_Argument(obj, dest=dest, nargs=nargs))
```
- _OptionParser.parse_args (src/click/parser.py:298-314)
```python
    def parse_args(
        self, args: list[str]
    ) -> tuple[dict[str, t.Any], list[str], list[CoreParameter]]:
        """Parses positional arguments and returns ``(values, args, order)``
        for the parsed options and arguments as well as the leftover
        arguments if there are any.  The order is a list of objects as they
        appear on the command line.  If arguments appear multiple times they
        will be memorized multiple times as well.
        """
        state = _ParsingState(args)
        try:
            self._process_args_for_options(state)
            self._process_args_for_args(state)
        except UsageError:
            if self.ctx is None or not self.ctx.resilient_parsing:
                raise
        return state.opts, state.largs, state.order
```
- option (src/click/decorators.py:352-377)
```python
def option(
    *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any
) -> t.Callable[[FC], FC]:
    """Attaches an option to the command.  All positional arguments are
    passed as parameter declarations to :class:`Option`; all keyword
    arguments are forwarded unchanged (except ``cls``).
    This is equivalent to creating an :class:`Option` instance manually
    and attaching it to the :attr:`Command.params` list.

    For the default option class, refer to :class:`Option` and
    :class:`Parameter` for descriptions of parameters.

    :param cls: the option class to instantiate.  This defaults to
                :class:`Option`.
    :param param_decls: Passed as positional arguments to the constructor of
        ``cls``.
    :param attrs: Passed as keyword arguments to the constructor of ``cls``.
    """
    if cls is None:
        cls = Option

    def decorator(f: FC) -> FC:
        _param_memo(f, cls(param_decls, **attrs))
        return f

    return decorator
```
- FishComplete (src/click/shell_completion.py:484-518)
```python
class FishComplete(ShellComplete):
    """Shell completion for Fish."""

    name: t.ClassVar[str] = "fish"
    source_template: t.ClassVar[str] = _SOURCE_FISH

    def get_completion_args(self) -> tuple[list[str], str]:
        cwords = split_arg_string(os.environ["COMP_WORDS"])
        incomplete = os.environ["COMP_CWORD"]
        if incomplete:
            incomplete = split_arg_string(incomplete)[0]
        args = cwords[1:]

        # Fish stores the partial word in both COMP_WORDS and
        # COMP_CWORD, remove it from complete args.
        if incomplete and args and args[-1] == incomplete:
            args.pop()

        return args, incomplete

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
- Command.get_help_option_names (src/click/core.py:1182-1195)
```python
    def get_help_option_names(self, ctx: Context) -> list[str]:
        """Returns the names for the help option.

        Drops duplicates and names already reserved by another parameter. Order of
        :attr:`Context.help_option_names` is preserved, so the result is stable.

        .. versionchanged:: 8.5.0
            Names keep their declaration order.
        """
        all_names = dict.fromkeys(ctx.help_option_names)
        for param in self.params:
            for name in (*param.opts, *param.secondary_opts):
                all_names.pop(name, None)
        return list(all_names)
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
- HelpFormatter.section (src/click/formatting.py:274-286)
```python
    def section(self, name: str) -> cabc.Generator[None]:
        """Helpful context manager that writes a paragraph, a heading,
        and the indents.

        :param name: the section name that is written as heading.
        """
        self.write_paragraph()
        self.write_heading(name)
        self.indent()
        try:
            yield
        finally:
            self.dedent()
```
- test_argument_help_optional_metavar (tests/test_arguments.py:327-336)
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
- Command (src/click/core.py:964-1659)
```python
class Command:
    """Commands are the basic building block of command line interfaces in
    Click.  A basic command handles command line parsing and might dispatch
    more parsing to commands nested below it.

    :param name: the name of the command to use unless a group overrides it.
    :param context_settings: an optional dictionary with defaults that are
                             passed to the context object.
    :param callback: the callback to invoke.  This is optional.
    :param params: the parameters to register with this command.  This can
                   be either :class:`Option` or :class:`Argument` objects.
    :param help: the help string to use for this command.
    :param epilog: like the help string but it's printed at the end of the
                   help page after everything else.
    :param short_help: the short help to use for this command.  This is
                       shown on the command listing of the parent command.
    :param add_help_option: by default each command registers a ``--help``
                            option.  This can be disabled by this parameter.
    :param no_args_is_help: this controls what happens if no arguments are
                            provided.  This option is disabled by default.
                            If enabled this will add ``--help`` as argument
                            if no arguments are passed
    :param hidden: hide this command from help outputs.
