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
