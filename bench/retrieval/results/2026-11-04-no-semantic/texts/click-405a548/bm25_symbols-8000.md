- test_group_with_args (tests/test_commands.py:129-141)
```python
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
```
- Command.get_params (src/click/core.py:1107-1161)
```python
    def get_params(self, ctx: Context) -> list[Parameter]:
        params = self.params
        help_option = self.get_help_option(ctx)

        if help_option is not None:
            params = [*params, help_option]

        if __debug__:
            import warnings

            opts = [opt for param in params for opt in param.opts]
            opts_counter = Counter(opts)
            duplicate_opts = (opt for opt, count in opts_counter.items() if count > 1)

            for duplicate_opt in duplicate_opts:
                warnings.warn(
                    (
                        f"The parameter {duplicate_opt} is used more than once. "
                        "Remove its duplicate as parameters should be unique."
                    ),
                    stacklevel=3,
                )

            # Options may deliberately share a storage name to compete for
            # the same value (feature switches), but an argument sharing a
            # name silently overwrites the other parameter's value.
            names_counter = Counter(param.name for param in params)
            duplicate_names = (
                name for name, count in names_counter.items() if count > 1
            )

            for duplicate_name in duplicate_names:
                sharers = [param for param in params if param.name == duplicate_name]

                if help_option in sharers:
                    warnings.warn(
                        (
                            f"The name {duplicate_name!r} is reserved for the "
                            "automatic help option. Give the parameter a "
                            "different name."
                        ),
                        stacklevel=3,
                    )
                elif any(isinstance(param, Argument) for param in sharers):
                    warnings.warn(
                        (
                            f"The name {duplicate_name!r} is used by an argument "
                            "and another parameter. They will overwrite each "
                            "other's value during parsing. Give each parameter "
                            "a unique name."
                        ),
                        stacklevel=3,
                    )

        return params
```
- ship_move (examples/naval/naval.py:32-34)
```python
def ship_move(ship, x, y, speed):
    """Moves SHIP to the new location X,Y."""
    click.echo(f"Moving ship {ship} to {x},{y} with speed {speed}")
```
- Command.get_help_option (src/click/core.py:1197-1229)
```python
    def get_help_option(self, ctx: Context) -> Option | None:
        """Returns the help option object.

        Skipped if :attr:`add_help_option` is ``False``.

        .. versionchanged:: 8.5.0
            The help option stores its value under the reserved name
            ``_click_default_help``, so a parameter named ``help`` no
            longer breaks parsing.

        .. versionchanged:: 8.1.8
            The help option is now cached to avoid creating it multiple times.
        """
        help_option_names = self.get_help_option_names(ctx)

        if not help_option_names or not self.add_help_option:
            return None

        # Cache the help option object in private _help_option attribute to
        # avoid creating it multiple times. Not doing this will break the
        # callback ordering by iter_params_for_processing(), which relies on
        # object comparison.
        if self._help_option is None:
            # Avoid circular import.
            from .decorators import help_option

            # The help option never exposes its value, so it uses a reserved
            # storage name, keeping it clear of user parameters (like an
            # argument named "help") that would otherwise clobber its value.
            help_option(*help_option_names, _HELP_OPTION_STORAGE_NAME)(self)
            self._help_option = self.params.pop()  # type: ignore[assignment]

        return self._help_option
```
- test_required_option (tests/test_options.py:960-971)
```python
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
```
- test_formatting_custom_type_metavar (tests/test_formatting.py:317-335)
```python
def test_formatting_custom_type_metavar(runner):
    class MyType(click.ParamType):
        def get_metavar(self, param: click.Parameter, ctx: click.Context):
            return "MY_TYPE"

    @click.command("foo")
    @click.help_option()
    @click.argument("param", type=MyType())
    def cmd(param):
        pass

    result = runner.invoke(cmd, "--help")
    assert not result.exception
    assert result.output.splitlines() == [
        "Usage: foo [OPTIONS] MY_TYPE",
        "",
        "Options:",
        "  --help  Show this message and exit.",
    ]
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
- test_option_flag (tests/test_shell_completion.py:213-224)
```python
def test_option_flag():
    cli = Command(
        "cli",
        add_help_option=False,
        params=[
            Option(["--on/--off"]),
            Argument(["a"], type=Choice(["a1", "a2", "b"])),
        ],
    )
    assert _get_words(cli, [], "--") == ["--on", "--off"]
    # flag option doesn't take value, use choice argument
    assert _get_words(cli, ["--on"], "a") == ["a1", "a2"]
```
- test_nested_subcommand_help (tests/test_arguments.py:644-663)
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
- test_param_squatting_help_option_name (tests/test_basic.py:73-87)
```python
def test_param_squatting_help_option_name(runner):
    """Claiming the reserved storage name of the automatic help option
    triggers a warning.
    """

    @click.command()
    @click.argument("_click_default_help")
    def cli(_click_default_help):
        click.echo(_click_default_help)

    with pytest.warns(UserWarning, match="reserved for the automatic help option"):
        result = runner.invoke(cli, ["value"])

    # The collision still breaks parsing, but no longer silently.
    assert result.exit_code == 2
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
- test_formatting_usage_error_help_hint (tests/test_formatting.py:285-314)
```python
def test_formatting_usage_error_help_hint(
    runner, help_names, extra_options, expected_hint
):
    """The error hint should only show non-shadowed help option names,
    picking the longest for readability.

    https://github.com/pallets/click/issues/2790
    """

    @click.group(context_settings={"help_option_names": help_names})
    def cli():
        pass

    @cli.command()
    @click.argument("required_arg")
    def foo(required_arg, **kwargs):
        pass

    for option in extra_options:
        option(foo)

    result = runner.invoke(cli, ["foo"])
    assert result.exit_code == 2
    lines = result.output.splitlines()
    assert lines[0] == "Usage: cli foo [OPTIONS] REQUIRED_ARG"
    assert lines[-1] == "Error: Missing argument 'REQUIRED_ARG'."
    if expected_hint is not None:
        assert expected_hint in lines
    else:
        assert not any(line.startswith("Try ") for line in lines)
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
- test_flag_option_with_nargs_option (tests/test_shell_completion.py:227-237)
```python
def test_flag_option_with_nargs_option():
    cli = Command(
        "cli",
        add_help_option=False,
        params=[
            Argument(["a"], type=Choice(["a1", "a2", "b"])),
            Option(["--flag"], is_flag=True),
            Option(["-c"], type=Choice(["p", "q"]), nargs=2),
        ],
    )
    assert _get_words(cli, ["a1", "--flag", "-c"], "") == ["p", "q"]
```
- help_option (src/click/decorators.py:603-627)
```python
def help_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
    """Pre-configured ``--help`` option which immediately prints the help page
    and exits the program.

    :param param_decls: One or more option names. Defaults to the single
        value ``"--help"``.
    :param kwargs: Extra arguments are passed to :func:`option`.
    """

    def show_help(ctx: Context, param: Parameter, value: bool) -> None:
        """Callback that print the help page on ``<stdout>`` and exits."""
        if value and not ctx.resilient_parsing:
            echo(ctx.get_help(), color=ctx.color)
            ctx.exit()

    if not param_decls:
        param_decls = ("--help",)

    kwargs.setdefault("is_flag", True)
    kwargs.setdefault("expose_value", False)
    kwargs.setdefault("is_eager", True)
    kwargs.setdefault("help", _("Show this message and exit."))
    kwargs.setdefault("callback", show_help)

    return option(*param_decls, **kwargs)
```
- MissingParameter (src/click/exceptions.py:159-229)
```python
class MissingParameter(BadParameter):
    """Raised if click required an option or argument but it was not
    provided when invoking the script.

    .. versionadded:: 4.0

    :param param_type: a string that indicates the type of the parameter.
                       The default is to inherit the parameter type from
                       the given `param`.  Valid values are ``'parameter'``,
                       ``'option'`` or ``'argument'``.
    """

    param_type: t.Final[str | None]

    def __init__(
        self,
        message: str | None = None,
        ctx: Context | None = None,
        param: Parameter | None = None,
        param_hint: cabc.Sequence[str] | str | None = None,
        param_type: str | None = None,
    ) -> None:
        super().__init__(message or "", ctx, param, param_hint)
        self.param_type = param_type

    def format_message(self) -> str:
        if self.param_hint is not None:
            param_hint: cabc.Sequence[str] | str | None = self.param_hint
        elif self.param is not None:
            param_hint = self.param.get_error_hint(self.ctx)
        else:
            param_hint = None

        param_hint = _join_param_hints(param_hint)
        param_hint = f" {param_hint}" if param_hint else ""

        param_type = self.param_type
        if param_type is None and self.param is not None:
            param_type = self.param.param_type_name

        msg = self.message
        if self.param is not None:
            msg_extra = self.param.type.get_missing_message(
                param=self.param, ctx=self.ctx
            )
            if msg_extra:
                if msg:
                    msg += f". {msg_extra}"
                else:
                    msg = msg_extra

        msg = f" {msg}" if msg else ""

        # Translate param_type for known types.
        if param_type == "argument":
            missing = _("Missing argument")
        elif param_type == "option":
            missing = _("Missing option")
        elif param_type == "parameter":
            missing = _("Missing parameter")
        else:
            missing = _("Missing {param_type}").format(param_type=param_type)

        return f"{missing}{param_hint}.{msg}"

    def __str__(self) -> str:
        if not self.message:
            param_name = self.param.name if self.param else None
            return _("Missing parameter: {param_name}").format(param_name=param_name)
        else:
            return self.message
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
    :param deprecated: If ``True`` or non-empty string, issues a message
                        indicating that the command is deprecated and highlights
                        its deprecation in --help. The message can be customized
                        by using a string as the value.

    .. versionchanged:: 8.2
        This is the base class for all commands, not ``BaseCommand``.
        ``deprecated`` can be set to a string as well to customize the
        deprecation message.

    .. versionchanged:: 8.1
        ``help``, ``epilog``, and ``short_help`` are stored unprocessed,
        all formatting is done when outputting help text, not at init,
        and is done even if not using the ``@command`` decorator.

    .. versionchanged:: 8.0
        Added a ``repr`` showing the command name.

    .. versionchanged:: 7.1
        Added the ``no_args_is_help`` parameter.

    .. versionchanged:: 2.0
        Added the ``context_settings`` parameter.
    """

    #: The context class to create with :meth:`make_context`.
    #:
    #: .. versionadded:: 8.0
    context_class: type[Context] = Context

    #: the default for the :attr:`Context.allow_extra_args` flag.
    allow_extra_args = False

    #: the default for the :attr:`Context.allow_interspersed_args` flag.
    allow_interspersed_args = True

    #: the default for the :attr:`Context.ignore_unknown_options` flag.
    ignore_unknown_options = False

    name: str | None
    context_settings: cabc.MutableMapping[str, t.Any]
    callback: t.Callable[..., t.Any] | None
    params: list[Parameter]
    help: str | None
    epilog: str | None
    options_metavar: str | None
    short_help: str | None
    add_help_option: bool
    _help_option: Option | None
    no_args_is_help: bool
    hidden: bool
    deprecated: bool | str

    def __init__(
        self,
        name: str | None,
        context_settings: cabc.MutableMapping[str, t.Any] | None = None,
        callback: t.Callable[..., t.Any] | None = None,
        params: list[Parameter] | None = None,
        help: str | None = None,
        epilog: str | None = None,
        short_help: str | None = None,
        options_metavar: str | None = "[OPTIONS]",
        add_help_option: bool = True,
        no_args_is_help: bool = False,
        hidden: bool = False,
        deprecated: bool | str = False,
    ) -> None:
        #: the name the command thinks it has.  Upon registering a command
        #: on a :class:`Group` the group will default the command name
        #: with this information.  You should instead use the
        #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
        self.name = name

        if context_settings is None:
            context_settings = {}

        #: an optional dictionary with defaults passed to the context.
        self.context_settings = context_settings

        #: the callback to execute when the command fires.  This might be
        #: `None` in which case nothing happens.
        self.callback = callback
        #: the list of parameters for this command in the order they
        #: should show up in the help page and execute.  Eager parameters
        #: will automatically be handled before non eager ones.
        self.params = params or []
        self.help = help
        self.epilog = epilog
        self.options_metavar = options_metavar
        self.short_help = short_help
        self.add_help_option = add_help_option
        self._help_option = None
        self.no_args_is_help = no_args_is_help
        self.hidden = hidden
        self.deprecated = deprecated

    def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
        return {
            "name": self.name,
            "params": [param.to_info_dict() for param in self.get_params(ctx)],
            "help": self.help,
            "epilog": self.epilog,
            "short_help": self.short_help,
            "hidden": self.hidden,
            "deprecated": self.deprecated,
        }

    def __repr__(self) -> str:
        return f"<{self.__class__.__name__} {self.name}>"

    def get_usage(self, ctx: Context) -> str:
        """Formats the usage line into a string and returns it.

        Calls :meth:`format_usage` internally.
        """
        formatter = ctx.make_formatter()
        self.format_usage(ctx, formatter)
        return formatter.getvalue().rstrip("\n")

    def get_params(self, ctx: Context) -> list[Parameter]:
        params = self.params
        help_option = self.get_help_option(ctx)

        if help_option is not None:
            params = [*params, help_option]

        if __debug__:
            import warnings

            opts = [opt for param in params for opt in param.opts]
            opts_counter = Counter(opts)
            duplicate_opts = (opt for opt, count in opts_counter.items() if count > 1)

            for duplicate_opt in duplicate_opts:
                warnings.warn(
                    (
                        f"The parameter {duplicate_opt} is used more than once. "
                        "Remove its duplicate as parameters should be unique."
                    ),
                    stacklevel=3,
                )

            # Options may deliberately share a storage name to compete for
            # the same value (feature switches), but an argument sharing a
            # name silently overwrites the other parameter's value.
            names_counter = Counter(param.name for param in params)
            duplicate_names = (
                name for name, count in names_counter.items() if count > 1
            )

            for duplicate_name in duplicate_names:
                sharers = [param for param in params if param.name == duplicate_name]

                if help_option in sharers:
                    warnings.warn(
                        (
                            f"The name {duplicate_name!r} is reserved for the "
                            "automatic help option. Give the parameter a "
                            "different name."
                        ),
                        stacklevel=3,
                    )
                elif any(isinstance(param, Argument) for param in sharers):
                    warnings.warn(
                        (
                            f"The name {duplicate_name!r} is used by an argument "
                            "and another parameter. They will overwrite each "
                            "other's value during parsing. Give each parameter "
                            "a unique name."
                        ),
                        stacklevel=3,
                    )

        return params

    def format_usage(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes the usage line into the formatter.

        This is a low-level method called by :meth:`get_usage`.
        """
        pieces = self.collect_usage_pieces(ctx)
        formatter.write_usage(ctx.command_path, " ".join(pieces))

    def collect_usage_pieces(self, ctx: Context) -> list[str]:
        """Returns all the pieces that go into the usage line and returns
        it as a list of strings.
        """
        rv = [self.options_metavar] if self.options_metavar else []

        for param in self.get_params(ctx):
            rv.extend(param.get_usage_pieces(ctx))

        return rv

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

    def get_help_option(self, ctx: Context) -> Option | None:
        """Returns the help option object.

        Skipped if :attr:`add_help_option` is ``False``.

        .. versionchanged:: 8.5.0
            The help option stores its value under the reserved name
            ``_click_default_help``, so a parameter named ``help`` no
            longer breaks parsing.

        .. versionchanged:: 8.1.8
            The help option is now cached to avoid creating it multiple times.
        """
        help_option_names = self.get_help_option_names(ctx)

        if not help_option_names or not self.add_help_option:
            return None

        # Cache the help option object in private _help_option attribute to
        # avoid creating it multiple times. Not doing this will break the
        # callback ordering by iter_params_for_processing(), which relies on
        # object comparison.
        if self._help_option is None:
            # Avoid circular import.
            from .decorators import help_option

            # The help option never exposes its value, so it uses a reserved
            # storage name, keeping it clear of user parameters (like an
            # argument named "help") that would otherwise clobber its value.
            help_option(*help_option_names, _HELP_OPTION_STORAGE_NAME)(self)
            self._help_option = self.params.pop()  # type: ignore[assignment]

        return self._help_option

    def make_parser(self, ctx: Context) -> _OptionParser:
        """Creates the underlying option parser for this command."""
        parser = _OptionParser(ctx)
        for param in self.get_params(ctx):
            param.add_to_parser(parser, ctx)
        return parser

    def get_help(self, ctx: Context) -> str:
        """Formats the help into a string and returns it.

        Calls :meth:`format_help` internally.
        """
        formatter = ctx.make_formatter()
        self.format_help(ctx, formatter)
        return formatter.getvalue().rstrip("\n")

    def get_short_help_str(self, limit: int = 45) -> str:
        """Gets short help for the command or makes it by shortening the
        long help string.
        """
        if self.short_help:
            text = inspect.cleandoc(self.short_help)
        elif self.help:
            text = _make_default_short_help(self.help, limit)
        else:
            text = ""

        if self.deprecated:
            text = f"{_(text)} {_format_deprecated_label(self.deprecated)}"

        return text.strip()

    def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes the help into the formatter if it exists.

        This is a low-level method called by :meth:`get_help`.

        This calls the following methods:

        -   :meth:`format_usage`
        -   :meth:`format_help_text`
        -   :meth:`format_arguments`
        -   :meth:`format_options`
        -   :meth:`format_epilog`
        """
        self.format_usage(ctx, formatter)
        self.format_help_text(ctx, formatter)
        self.format_arguments(ctx, formatter)
        self.format_options(ctx, formatter)
        self.format_epilog(ctx, formatter)

    def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes the help text to the formatter if it exists."""
        if self.help is not None:
            # truncate the help text to the first form feed
            text = inspect.cleandoc(self.help).partition("\f")[0]
        else:
            text = ""

        if self.deprecated:
            label = _format_deprecated_label(self.deprecated)
            text = f"{_(text)} {label}" if text else label

        if text:
            formatter.write_paragraph()

            with formatter.indentation():
                formatter.write_text(text)

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

    def format_epilog(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes the epilog into the formatter if it exists."""
        if self.epilog:
            epilog = inspect.cleandoc(self.epilog)
            formatter.write_paragraph()

            with formatter.indentation():
                formatter.write_text(epilog)

    def make_context(
        self,
        info_name: str | None,
        args: list[str],
        parent: Context | None = None,
        **extra: t.Any,
    ) -> Context:
        """This function when given an info name and arguments will kick
        off the parsing and create a new :class:`Context`.  It does not
        invoke the actual command callback though.

        To quickly customize the context class used without overriding
        this method, set the :attr:`context_class` attribute.

        :param info_name: the info name for this invocation.  Generally this
                          is the most descriptive name for the script or
                          command.  For the toplevel script it's usually
                          the name of the script, for commands below it's
                          the name of the command.
        :param args: the arguments to parse as list of strings.
        :param parent: the parent context if available.
        :param extra: extra keyword arguments forwarded to the context
                      constructor.

        .. versionchanged:: 8.0
            Added the :attr:`context_class` attribute.
        """
        for key, value in self.context_settings.items():
            if key not in extra:
                extra[key] = value

        ctx = self.context_class(self, info_name=info_name, parent=parent, **extra)

        with ctx.scope(cleanup=False):
            self.parse_args(ctx, args)
        return ctx

    def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
        if not args and self.no_args_is_help and not ctx.resilient_parsing:
            raise NoArgsIsHelpError(ctx)

        parser = self.make_parser(ctx)
        opts, args, param_order = parser.parse_args(args=args)

        for param in iter_params_for_processing(param_order, self.get_params(ctx)):
            _, args = param.handle_parse_result(ctx, opts, args)

        # We now have all parameters' values into `ctx.params`, but the data may contain
        # the `UNSET` sentinel.
        # Convert `UNSET` to `None` to ensure that the user doesn't see `UNSET`.
        #
