- test_flag_value_dual_options (tests/test_basic.py:395-415)
```python
def test_flag_value_dual_options(runner, default, args, expected):
    """Check how default is processed when options compete for the same variable name.

    Covers the regression reported in
    https://github.com/pallets/click/issues/3024#issuecomment-3146199461

    .. hint::
        Similar to ``test_options.py::test_default_dual_option_callback``.

        ``test_defaults.py::test_shared_param_prefers_first_default``
        is a smoke-test complement that exercises both default placements.
    """

    @click.command()
    @click.option("--upper", "case", flag_value="upper", default=default)
    @click.option("--lower", "case", flag_value="lower")
    def cli(case):
        click.echo(repr(case), nl=False)

    result = runner.invoke(cli, args)
    assert result.output == repr(expected)
```
- Choice.shell_complete (src/click/types.py:495-515)
```python
    def shell_complete(
        self, ctx: Context, param: Parameter, incomplete: str
    ) -> list[CompletionItem]:
        """Complete choices that start with the incomplete value.

        :param ctx: Invocation context for this command.
        :param param: The parameter that is requesting completion.
        :param incomplete: Value being completed. May be empty.

        .. versionadded:: 8.0
        """
        from click.shell_completion import CompletionItem

        str_choices = [self.normalize_choice(choice, ctx) for choice in self.choices]
        if self.case_sensitive:
            matched = (c for c in str_choices if c.startswith(incomplete))
        else:
            incomplete = incomplete.lower()
            matched = (c for c in str_choices if c.lower().startswith(incomplete))

        return [CompletionItem(c) for c in matched]
```
- Argument._parse_decls (src/click/core.py:3778-3795)
```python
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
```
- _posixify (src/click/utils.py:32-33)
```python
def _posixify(name: str) -> str:
    return "-".join(name.split()).lower()
```
- Choice (src/click/types.py:342-515)
```python
class Choice(ParamType[_ValueT_co], t.Generic[_ValueT_co]):
    """The choice type allows a value to be checked against a fixed set
    of supported values.

    You may pass any iterable value which will be converted to a tuple
    and thus will only be iterated once.

    The resulting value will always be one of the originally passed choices.
    See :meth:`normalize_choice` for more info on the mapping of strings
    to choices. See :ref:`choice-opts` for an example.

    :param case_sensitive: Set to false to make choices case
        insensitive. Defaults to true.

    .. versionchanged:: 8.4.0
        Now generic in the choice value type. Parameterize with the type of
        the choice values (``Choice[HashType]`` for an enum, ``Choice[str]``
        for plain strings) to enable type-checked consumers.

    .. versionchanged:: 8.2.0
        Non-``str`` ``choices`` are now supported. It can additionally be any
        iterable. Before you were not recommended to pass anything but a list or
        tuple.

    .. versionadded:: 8.2.0
        Choice normalization can be overridden via :meth:`normalize_choice`.
    """

    name: str = "choice"

    choices: tuple[_ValueT_co, ...]
    case_sensitive: bool

    def __init__(
        self, choices: cabc.Iterable[_ValueT_co], case_sensitive: bool = True
    ) -> None:
        self.choices = tuple(choices)
        self.case_sensitive = case_sensitive

    def to_info_dict(self) -> ChoiceInfoDict[_ValueT_co]:
        return {
            "choices": self.choices,
            "case_sensitive": self.case_sensitive,
            **super().to_info_dict(),
        }

    def _normalized_mapping(
        self, ctx: Context | None = None
    ) -> cabc.Mapping[_ValueT_co, str]:
        """
        Returns mapping where keys are the original choices and the values are
        the normalized values that are accepted via the command line.

        This is a simple wrapper around :meth:`normalize_choice`, use that
        instead which is supported.
        """
        return {
            choice: self.normalize_choice(
                choice=choice,
                ctx=ctx,
            )
            for choice in self.choices
        }

    def normalize_choice(self, choice: object, ctx: Context | None) -> str:
        """
        Normalize a choice value, used to map a passed string to a choice.
        Each choice must have a unique normalized value.

        By default uses :meth:`Context.token_normalize_func` and if not case
        sensitive, convert it to a casefolded value.

        .. versionadded:: 8.2.0
        """
        normed_value = choice.name if isinstance(choice, enum.Enum) else str(choice)

        if ctx is not None and ctx.token_normalize_func is not None:
            normed_value = ctx.token_normalize_func(normed_value)

        if not self.case_sensitive:
            normed_value = normed_value.casefold()

        return normed_value

    def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
        if param.param_type_name == "option" and not param.show_choices:  # type: ignore[attr-defined]
            choice_metavars = [
                convert_type(type(choice)).name.upper() for choice in self.choices
            ]
            choices_str = "|".join([*dict.fromkeys(choice_metavars)])
        else:
            choices_str = "|".join(
                [str(i) for i in self._normalized_mapping(ctx=ctx).values()]
            )

        # Use curly braces to indicate a required argument.
        if param.required and param.param_type_name == "argument":
            return f"{{{choices_str}}}"

        # Use square braces to indicate an option or optional argument.
        return f"[{choices_str}]"

    def get_missing_message(self, param: Parameter, ctx: Context | None) -> str:
        """
        Message shown when no choice is passed.

        .. versionchanged:: 8.2.0 Added ``ctx`` argument.
        """
        return _("Choose from:\n\t{choices}").format(
            choices=",\n\t".join(self._normalized_mapping(ctx=ctx).values())
        )

    def convert(
        self, value: t.Any, param: Parameter | None, ctx: Context | None
    ) -> _ValueT_co:
        """
        For a given value from the parser, normalize it and find its
        matching normalized value in the list of choices. Then return the
        matched "original" choice.
        """
        normed_value = self.normalize_choice(choice=value, ctx=ctx)
        normalized_mapping = self._normalized_mapping(ctx=ctx)

        try:
            return next(
                original
                for original, normalized in normalized_mapping.items()
                if normalized == normed_value
            )
        except StopIteration:
            self.fail(
                self.get_invalid_choice_message(value=value, ctx=ctx),
                param=param,
                ctx=ctx,
            )

    def get_invalid_choice_message(self, value: t.Any, ctx: Context | None) -> str:
        """Get the error message when the given choice is invalid.

        :param value: The invalid value.

        .. versionadded:: 8.2
        """
        choices_str = ", ".join(map(repr, self._normalized_mapping(ctx=ctx).values()))
        return ngettext(
            "{value!r} is not {choice}.",
            "{value!r} is not one of {choices}.",
            len(self.choices),
        ).format(value=value, choice=choices_str, choices=choices_str)

    def __repr__(self) -> str:
        return _("Choice({choices})").format(choices=list(self.choices))

    def shell_complete(
        self, ctx: Context, param: Parameter, incomplete: str
    ) -> list[CompletionItem]:
        """Complete choices that start with the incomplete value.

        :param ctx: Invocation context for this command.
        :param param: The parameter that is requesting completion.
        :param incomplete: Value being completed. May be empty.

        .. versionadded:: 8.0
        """
        from click.shell_completion import CompletionItem

        str_choices = [self.normalize_choice(choice, ctx) for choice in self.choices]
        if self.case_sensitive:
            matched = (c for c in str_choices if c.startswith(incomplete))
        else:
            incomplete = incomplete.lower()
            matched = (c for c in str_choices if c.lower().startswith(incomplete))

        return [CompletionItem(c) for c in matched]
```
- test_options_sharing_name_no_warning (tests/test_options.py:3450-3464)
```python
def test_options_sharing_name_no_warning(runner):
    """Options deliberately sharing a storage name (feature switches) are
    exempt from the duplicate-name warning. Warnings are errors in this
    suite, so a clean run proves none fired.
    """

    @click.command()
    @click.option("--upper", "transformation", flag_value="upper")
    @click.option("--lower", "transformation", flag_value="lower")
    def cli(transformation):
        click.echo(transformation, nl=False)

    result = runner.invoke(cli, ["--upper"])
    assert result.output == "upper"
    assert result.exit_code == 0
```
- BoolParamType (src/click/types.py:819-885)
```python
class BoolParamType(ParamType[bool]):
    name = "boolean"

    bool_states: dict[str, bool] = {
        "1": True,
        "0": False,
        "yes": True,
        "no": False,
        "true": True,
        "false": False,
        "on": True,
        "off": False,
        "t": True,
        "f": False,
        "y": True,
        "n": False,
        # Absence of value is considered False.
        "": False,
    }
    """A mapping of string values to boolean states.

    Mapping is inspired by :py:attr:`configparser.ConfigParser.BOOLEAN_STATES`
    and extends it.

    .. caution::
        String values are lower-cased, as the ``str_to_bool`` comparison function
        below is case-insensitive.

    .. warning::
        The mapping is not exhaustive, and does not cover all possible boolean strings
        representations. It will remains as it is to avoid endless bikeshedding.

        Future work my be considered to make this mapping user-configurable from public
        API.
    """

    @staticmethod
    def str_to_bool(value: str | bool) -> bool | None:
        """Convert a string to a boolean value.

        If the value is already a boolean, it is returned as-is. If the value is a
        string, it is stripped of whitespaces and lower-cased, then checked against
        the known boolean states pre-defined in the `BoolParamType.bool_states` mapping
        above.

        Returns `None` if the value does not match any known boolean state.
        """
        if isinstance(value, bool):
            return value
        return BoolParamType.bool_states.get(value.strip().lower())

    def convert(
        self, value: t.Any, param: Parameter | None, ctx: Context | None
    ) -> bool:
        normalized = self.str_to_bool(value)
        if normalized is None:
            self.fail(
                _(
                    "{value!r} is not a valid boolean. Recognized values: {states}"
                ).format(value=value, states=", ".join(sorted(self.bool_states))),
                param,
                ctx,
            )
        return normalized

    def __repr__(self) -> str:
        return "BOOL"
```
- convert_flip (examples/imagepipe/imagepipe.py:178-186)
```python
def convert_flip(ctx, param, value):
    if value is None:
        return
    value = value.lower()
    if value in ("lr", "leftright"):
        return (Image.FLIP_LEFT_RIGHT, "left to right")
    if value in ("tb", "topbottom", "upsidedown", "ud"):
        return (Image.FLIP_LEFT_RIGHT, "top to bottom")
    raise click.BadParameter(f"invalid flip '{value}'")
```
- convert_rotation (examples/imagepipe/imagepipe.py:165-175)
```python
def convert_rotation(ctx, param, value):
    if value is None:
        return
    value = value.lower()
    if value in ("90", "r", "right"):
        return (Image.ROTATE_90, 90)
    if value in ("180", "-180"):
        return (Image.ROTATE_180, 180)
    if value in ("-90", "270", "l", "left"):
        return (Image.ROTATE_270, 270)
    raise click.BadParameter(f"invalid rotation '{value}'")
```
- test_flag_group_competition_four_flags (tests/test_options.py:3539-3551)
```python
def test_flag_group_competition_four_flags(runner, opts, args, expected):
    """Arbitration rules applies to groups of any size."""

    @click.command()
    def cli(case):
        click.echo(repr(case), nl=False)

    for opt_name, opt_kwargs in opts:
        cli = click.option(opt_name, "case", **opt_kwargs)(cli)

    result = runner.invoke(cli, args)
    assert result.exit_code == 0, result.output
    assert result.output == repr(expected)
```
- AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
```python
    def get_command(self, ctx, cmd_name):
        # Step one: built-in commands as normal
        rv = click.Group.get_command(self, ctx, cmd_name)
        if rv is not None:
            return rv

        # Step two: find the config object and ensure it's there.  This
        # will create the config object is missing.
        cfg = ctx.ensure_object(Config)

        # Step three: look up an explicit command alias in the config
        if cmd_name in cfg.aliases:
            actual_cmd = cfg.aliases[cmd_name]
            return click.Group.get_command(self, ctx, actual_cmd)

        # Alternative option: if we did not find an explicit alias we
        # allow automatic abbreviation of the command.  "status" for
        # instance will match "st".  We only allow that however if
        # there is only one command.
        matches = [
            x for x in self.list_commands(ctx) if x.lower().startswith(cmd_name.lower())
        ]
        if not matches:
            return None
        elif len(matches) == 1:
            return click.Group.get_command(self, ctx, matches[0])
        ctx.fail(f"Too many matches: {', '.join(sorted(matches))}")
```
- test_flag_group_competition_non_boolean (tests/test_options.py:3340-3354)
```python
def test_flag_group_competition_non_boolean(runner, opts, args, expected):
    """Same arbitration rules as :func:`test_bool_flag_group_competition`,
    but for feature-switch groups with non-boolean ``flag_value``.
    """

    @click.command()
    def cli(case):
        click.echo(repr(case), nl=False)

    for opt_name, opt_kwargs in opts:
        cli = click.option(opt_name, "case", **opt_kwargs)(cli)

    result = runner.invoke(cli, args)
    assert result.exit_code == 0, result.output
    assert result.output == repr(expected)
```
- AliasedGroup (examples/aliases/aliases.py:37-73)
```python
class AliasedGroup(click.Group):
    """This subclass of a group supports looking up aliases in a config
    file and with a bit of magic.
    """

    def get_command(self, ctx, cmd_name):
        # Step one: built-in commands as normal
        rv = click.Group.get_command(self, ctx, cmd_name)
        if rv is not None:
            return rv

        # Step two: find the config object and ensure it's there.  This
        # will create the config object is missing.
        cfg = ctx.ensure_object(Config)

        # Step three: look up an explicit command alias in the config
        if cmd_name in cfg.aliases:
            actual_cmd = cfg.aliases[cmd_name]
            return click.Group.get_command(self, ctx, actual_cmd)

        # Alternative option: if we did not find an explicit alias we
        # allow automatic abbreviation of the command.  "status" for
        # instance will match "st".  We only allow that however if
        # there is only one command.
        matches = [
            x for x in self.list_commands(ctx) if x.lower().startswith(cmd_name.lower())
        ]
        if not matches:
            return None
        elif len(matches) == 1:
            return click.Group.get_command(self, ctx, matches[0])
        ctx.fail(f"Too many matches: {', '.join(sorted(matches))}")

    def resolve_command(self, ctx, args):
        # always return the command's name, not the alias
        _, cmd, args = super().resolve_command(ctx, args)
        return cmd.name, cmd, args
```
- test_file_surrogates (tests/test_types/test_File.py:13-31)
```python
def test_file_surrogates(type, tmp_path):
    """Ensures that the error handling in ``click.File`` is robust.

    ``EILSEQ`` shows up with rootless Podman (FUSE-backed paths) and on filesystems
    that reject non-UTF-8 names, like ZFS with ``utf8only=on``.

    See: https://github.com/pallets/click/issues/2634
    """
    path = tmp_path / "\udcff"
    match = (
        # Common case: �': No such file or directory.
        r"(�': No such file or directory"
        # BSD/macOS libc special case (EILSEQ).
        r"|Illegal byte sequence"
        # glibc special case (EILSEQ).
        r"|Invalid or incomplete multibyte or wide character)"
    )
    with pytest.raises(click.BadParameter, match=match):
        type.convert(path, None, None)
```
- Context.set_parameter_source (src/click/core.py:936-943)
```python
    def set_parameter_source(self, name: str, source: ParameterSource) -> None:
        """Set the source of a parameter. This indicates the location
        from which the value of the parameter was obtained.

        :param name: The name of the parameter.
        :param source: A member of :class:`~click.core.ParameterSource`.
        """
        self._parameter_source[name] = source
```
- CONTEXT_SETTINGS (tests/test_normalization.py:3-3)
```python
CONTEXT_SETTINGS = dict(token_normalize_func=lambda x: x.lower())
```
- Context.get_parameter_source (src/click/core.py:945-961)
```python
    def get_parameter_source(self, name: str) -> ParameterSource | None:
        """Get the source of a parameter. This indicates the location
        from which the value of the parameter was obtained.

        This can be useful for determining when a user specified a value
        on the command line that is the same as the default value. It
        will be :attr:`~click.core.ParameterSource.DEFAULT` only if the
        value was actually taken from the default.

        :param name: The name of the parameter.
        :rtype: ParameterSource

        .. versionchanged:: 8.0
            Returns ``None`` if the parameter was not provided from any
            source.
        """
        return self._parameter_source.get(name)
```
- test_choice_normalization (tests/test_normalization.py:18-50)
```python
def test_choice_normalization(runner):
    @click.command(context_settings=CONTEXT_SETTINGS)
    @click.option(
        "--method",
        type=click.Choice(
            ["SCREAMING_SNAKE_CASE", "snake_case", "PascalCase", "kebab-case"],
            case_sensitive=False,
        ),
    )
    def cli(method):
        click.echo(method)

    result = runner.invoke(cli, ["--METHOD=snake_case"])
    assert not result.exception, result.output
    assert result.output == "snake_case\n"

    # Even though it's case sensitive, the choice's original value is preserved
    result = runner.invoke(cli, ["--method=pascalcase"])
    assert not result.exception, result.output
    assert result.output == "PascalCase\n"

    result = runner.invoke(cli, ["--method=meh"])
    assert result.exit_code == 2
    assert (
        "Invalid value for '--method': 'meh' is not one of "
        "'screaming_snake_case', 'snake_case', 'pascalcase', 'kebab-case'."
    ) in result.output

    result = runner.invoke(cli, ["--help"])
    assert (
        "--method [screaming_snake_case|snake_case|pascalcase|kebab-case]"
        in result.output
    )
```
- Parameter.human_readable_name (src/click/core.py:2462-2466)
```python
    def human_readable_name(self) -> str:
        """Returns the human readable name of this parameter.  This is the
        same as the name for options, but the metavar for arguments.
        """
        return self.name
```
- test_parameter_duplication (tests/test_utils/test_sentinel.py:90-97)
```python
def test_parameter_duplication(duplicate):
    """Every ``Parameter`` holds ``UNSET`` in ``default`` unless one is given."""
    option = click.Option(["--name"])
    assert option.default is UNSET

    duplicated = duplicate(option)
    assert duplicated.default is UNSET
    assert duplicated.name == "name"
```
- MissingParameter.__str__ (src/click/exceptions.py:224-229)
```python
    def __str__(self) -> str:
        if not self.message:
            param_name = self.param.name if self.param else None
            return _("Missing parameter: {param_name}").format(param_name=param_name)
        else:
            return self.message
```
- test_default_map_source (tests/test_defaults.py:254-272)
```python
def test_default_map_source(runner, args, default_map, expected_value, expected_source):
    """``get_parameter_source()`` reports the correct origin for a parameter
    value across the resolution chain: CLI > default_map > parameter default.
    """

    @click.command()
    @click.option("--name", default="original")
    @click.pass_context
    def cli(ctx, name):
        source = ctx.get_parameter_source("name")
        click.echo(f"name={name!r} source={source.name}")

    kwargs = {}
    if default_map is not None:
        kwargs["default_map"] = default_map
    result = runner.invoke(cli, args, **kwargs)
    assert not result.exception
    assert f"name={expected_value!r}" in result.output
    assert f"source={expected_source}" in result.output
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
- Context.lookup_default (src/click/core.py:790-810)
```python
    def lookup_default(self, name: str, call: bool = True) -> t.Any | None:
        """Get the default for a parameter from :attr:`default_map`.

        :param name: Name of the parameter.
        :param call: If the default is a callable, call it. Disable to
            return the callable instead.

        .. versionchanged:: 8.0
            Added the ``call`` parameter.
        """
        if not self._default_map_has(name):
            return None

        # Assert to make the type checker happy.
        assert self.default_map is not None
        value = self.default_map[name]

        if call and callable(value):
            return value()

        return value
```
- Argument (src/click/core.py:3716-3819)
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

        super().__init__(param_decls, required=required, help=help, **attrs)

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

    def get_help_record(self, ctx: Context) -> tuple[str, str]:
        """Returns the argument's help row: its metavar and its help text.

        Unlike :meth:`Option.get_help_record`, this never returns ``None``. An
        argument cannot be hidden, so an undocumented one still gets a row, with
        an empty description.

        .. versionchanged:: 8.5.1
            Always returns a tuple. It used to return ``None`` when ``help`` was
            not set.
        """
        return self.make_metavar(ctx), self.help or ""

    def get_error_hint(self, ctx: Context | None) -> str:
        if ctx is not None:
            return f"'{self.make_metavar(ctx)}'"
        return f"'{self.human_readable_name}'"

    def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
        parser.add_argument(dest=self.name, nargs=self.nargs, obj=self)
```
- Parameter.__repr__ (src/click/core.py:2453-2454)
```python
    def __repr__(self) -> str:
        return f"<{self.__class__.__name__} {self.name}>"
```
- test_bool_flag_group_parameter_source (tests/test_options.py:3055-3075)
```python
def test_bool_flag_group_parameter_source(
    runner, opts, args, invoke_kwargs, expected_value, expected_source
):
    """``get_parameter_source()`` stays correct for feature-switch groups.

    Regression test for https://github.com/pallets/click/issues/3458.
    """

    @click.command()
    @click.pass_context
    def cli(ctx, enable_xyz):
        source = ctx.get_parameter_source("enable_xyz")
        click.echo(f"value={enable_xyz!r} source={source.name}")

    for opt_name, opt_kwargs in opts:
        cli = click.option(opt_name, "enable_xyz", **opt_kwargs)(cli)

    result = runner.invoke(cli, args, **invoke_kwargs)
    assert result.exit_code == 0, result.output
    assert f"value={expected_value!r}" in result.output
    assert f"source={expected_source}" in result.output
```
- test_parameter_source_during_eager_callback (tests/test_defaults.py:318-348)
```python
def test_parameter_source_during_eager_callback(runner):
    """``get_parameter_source()`` is available during eager callbacks.

    Regression test for https://github.com/pallets/click/issues/3458.
    """

    def eager_cb(ctx, param, value):
        source = ctx.get_parameter_source(param.name)
        click.echo(f"callback source={source.name if source else None}")

    @click.command()
    @click.option(
        "--flag/--no-flag",
        default=False,
        is_eager=True,
        callback=eager_cb,
        expose_value=False,
    )
    def cli():
        source = click.get_current_context().get_parameter_source("flag")
        click.echo(f"final source={source.name}")

    result = runner.invoke(cli, [])
    assert not result.exception
    assert "callback source=DEFAULT" in result.output
    assert "final source=DEFAULT" in result.output

    result = runner.invoke(cli, ["--flag"])
    assert not result.exception
    assert "callback source=COMMANDLINE" in result.output
    assert "final source=COMMANDLINE" in result.output
```
- Command.__init__ (src/click/core.py:1040-1082)
```python
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
```
- test_case_insensitive_choice_returned_exactly (tests/test_options.py:1054-1062)
```python
def test_case_insensitive_choice_returned_exactly(runner):
    @click.command()
    @click.option("--foo", type=click.Choice(["Orange", "Apple"], case_sensitive=False))
    def cmd(foo):
        click.echo(foo)

    result = runner.invoke(cmd, ["--foo", "apple"])
    assert result.exit_code == 0
    assert result.output == "Apple\n"
```
- Option._parse_decls (src/click/core.py:3277-3336)
```python
    def _parse_decls(
        self, decls: cabc.Sequence[str], expose_value: bool
    ) -> tuple[str, list[str], list[str]]:
        opts = []
        secondary_opts = []
        name = None
        possible_names = []

        for decl in decls:
            if decl.isidentifier():
                if name is not None:
                    raise TypeError(_("Name '{name}' defined twice").format(name=name))
                name = decl
            else:
                split_char = ";" if decl[:1] == "/" else "/"
                if split_char in decl:
                    first, second = decl.split(split_char, 1)
                    first = first.rstrip()
                    if first:
                        possible_names.append(_split_opt(first))
                        opts.append(first)
                    second = second.lstrip()
                    if second:
                        secondary_opts.append(second.lstrip())
                    if first == second:
                        raise ValueError(
                            _(
                                "Boolean option {decl!r} cannot use the"
                                " same flag for true/false."
                            ).format(decl=decl)
                        )
                else:
                    possible_names.append(_split_opt(decl))
                    opts.append(decl)

        if name is None and possible_names:
            possible_names.sort(key=lambda x: -len(x[0]))  # group long options first
            name = possible_names[0][1].replace("-", "_").lower()
            if not name.isidentifier():
                name = None

        if name is None:
            if not expose_value:
