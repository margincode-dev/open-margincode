- test_prompt_default_round_trips_through_type (tests/test_utils/test_prompt.py:17-31)
```python
def test_prompt_default_round_trips_through_type(capfd, monkeypatch):
    """A default that already has the converted value type still passes
    through ``ParamType.convert``, so conversion side effects keep
    applying to defaults."""

    class DoublingType(click.ParamType[int]):
        name = "doubling"

        def convert(self, value, param, ctx):
            return int(value) * 2

    monkeypatch.setattr(sys, "stdin", StringIO("\n"))
    value = click.prompt("value", default=5, type=DoublingType())
    capfd.readouterr()
    assert value == 10
```
- ParamType.convert (src/click/types.py:168-192)
```python
    def convert(
        self, value: _InputT_contra, param: Parameter | None, ctx: Context | None
    ) -> _ValueT_co:
        """Convert the value to the correct type. This is not called if
        the value is ``None`` (the missing value).

        This must accept string values from the command line, as well as
        values that are already the correct type. It may also convert
        other compatible types.

        The ``param`` and ``ctx`` arguments may be ``None`` in certain
        situations, such as when converting prompt input.

        If the value cannot be converted, call :meth:`fail` with a
        descriptive message.

        :param value: The value to convert.
        :param param: The parameter that is using this type to convert
            its value. May be ``None``.
        :param ctx: The current context that arrived at this value. May
            be ``None``.
        """
        # The default returns the value as-is so subclasses that only customize
        # metadata are not forced to redeclare ``convert``.
        return t.cast("_ValueT_co", value)
```
- DateTime.convert (src/click/types.py:555-576)
```python
    def convert(
        self, value: t.Any, param: Parameter | None, ctx: Context | None
    ) -> datetime:
        if isinstance(value, datetime):
            return value

        for format in self.formats:
            converted = self._try_to_convert_date(value, format)

            if converted is not None:
                return converted

        formats_str = ", ".join(map(repr, self.formats))
        self.fail(
            ngettext(
                "{value!r} does not match the format {format}.",
                "{value!r} does not match the formats {formats}.",
                len(self.formats),
            ).format(value=value, format=formats_str, formats=formats_str),
            param,
            ctx,
        )
```
- DoublingType.convert (tests/typing/typing_prompt.py:35-38)
```python
    def convert(
        self, value: str, param: click.Parameter | None, ctx: click.Context | None
    ) -> int:
        return int(value) * 2
```
- DoublingType (tests/typing/typing_prompt.py:32-38)
```python
class DoublingType(click.ParamType[int, str]):
    name = "doubling"

    def convert(
        self, value: str, param: click.Parameter | None, ctx: click.Context | None
    ) -> int:
        return int(value) * 2
```
- ParamType (src/click/types.py:54-228)
```python
class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
    """Represents the type of a parameter. Validates and converts values
    from the command line or Python into the correct type.

    To implement a custom type, subclass and implement at least the
    following:

    -   The :attr:`name` class attribute must be set.
    -   Calling an instance of the type with ``None`` must return
        ``None``. This is already implemented by default.
    -   :meth:`convert` must convert string values to the correct type.
    -   :meth:`convert` must accept values that are already the correct
        type.
    -   It must be able to convert a value if the ``ctx`` and ``param``
        arguments are ``None``. This can occur when converting prompt
        input.

    .. versionchanged:: 8.4.0
        Now a generic abstract base class. Parameterize with the
        converted value type (``ParamType[int]`` for an integer-returning
        type) so that :meth:`convert` and downstream consumers carry the
        narrowed return type.

    .. versionchanged:: 8.5.0
        Accepts a second optional type parameter for the input value type
        that :meth:`convert` accepts (``ParamType[int, str]`` for a type
        converting strings to integers), defaulting to ``Any``.
    """

    is_composite: t.ClassVar[bool] = False
    arity: int = 1  # read-only

    #: the descriptive name of this type
    name: str

    #: if a list of this type is expected and the value is pulled from a
    #: string environment variable, this is what splits it up.  `None`
    #: means any whitespace.  For all parameters the general rule is that
    #: whitespace splits them up.  The exception are paths and files which
    #: are split by ``os.path.pathsep`` by default (":" on Unix and ";" on
    #: Windows).
    envvar_list_splitter: t.ClassVar[str | None] = None

    if sys.version_info < (3, 13):
        # ``_InputT_contra`` carries its ``Any`` default only for type
        # checkers: ``TypeVar(default=...)`` (PEP 696) is unavailable at
        # runtime before Python 3.13. Fill in the omitted input type
        # parameter by hand so ``ParamType[int]`` keeps working.
        def __class_getitem__(cls, params: t.Any) -> t.Any:
            if cls is ParamType:
                if not isinstance(params, tuple):
                    params = (params,)
                if len(params) == 1:
                    params = (*params, t.Any)
            # Checkers cannot see Generic.__class_getitem__ through super().
            return super().__class_getitem__(params)  # type: ignore[misc]

    def to_info_dict(self) -> ParamTypeInfoDict:
        """Gather information that could be useful for a tool generating
        user-facing documentation.

        Use :meth:`click.Context.to_info_dict` to traverse the entire
        CLI structure.

        .. versionadded:: 8.0
        """
        # The class name without the "ParamType" suffix.
        param_type = type(self).__name__.partition("ParamType")[0]
        param_type = param_type.partition("ParameterType")[0]

        # Custom subclasses might not remember to set a name.
        if hasattr(self, "name"):
            name = self.name
        else:
            name = param_type

        return {"param_type": param_type, "name": name}

    @t.overload
    def __call__(
        self,
        value: None,
        param: Parameter | None = None,
        ctx: Context | None = None,
    ) -> None: ...

    @t.overload
    def __call__(
        self,
        value: _InputT_contra,
        param: Parameter | None = None,
        ctx: Context | None = None,
    ) -> _ValueT_co: ...

    def __call__(
        self,
        value: _InputT_contra | None,
        param: Parameter | None = None,
        ctx: Context | None = None,
    ) -> _ValueT_co | None:
        if value is not None:
            return self.convert(value, param, ctx)
        return None

    def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
        """Returns the metavar default for this param if it provides one."""

    def get_missing_message(self, param: Parameter, ctx: Context | None) -> str | None:
        """Optionally might return extra information about a missing
        parameter.

        .. versionadded:: 2.0
        """

    def convert(
        self, value: _InputT_contra, param: Parameter | None, ctx: Context | None
    ) -> _ValueT_co:
        """Convert the value to the correct type. This is not called if
        the value is ``None`` (the missing value).

        This must accept string values from the command line, as well as
        values that are already the correct type. It may also convert
        other compatible types.

        The ``param`` and ``ctx`` arguments may be ``None`` in certain
        situations, such as when converting prompt input.

        If the value cannot be converted, call :meth:`fail` with a
        descriptive message.

        :param value: The value to convert.
        :param param: The parameter that is using this type to convert
            its value. May be ``None``.
        :param ctx: The current context that arrived at this value. May
            be ``None``.
        """
        # The default returns the value as-is so subclasses that only customize
        # metadata are not forced to redeclare ``convert``.
        return t.cast("_ValueT_co", value)

    def split_envvar_value(self, rv: str) -> cabc.Sequence[str]:
        """Given a value from an environment variable this splits it up
        into small chunks depending on the defined envvar list splitter.

        If the splitter is set to `None`, which means that whitespace splits,
        then leading and trailing whitespace is ignored.  Otherwise, leading
        and trailing splitters usually lead to empty items being included.
        """
        return (rv or "").split(self.envvar_list_splitter)

    def fail(
        self,
        message: str,
        param: Parameter | None = None,
        ctx: Context | None = None,
    ) -> t.NoReturn:
        """Helper method to fail with an invalid value message."""
        raise BadParameter(message, ctx=ctx, param=param)

    def shell_complete(
        self, ctx: Context, param: Parameter, incomplete: str
    ) -> list[CompletionItem]:
        """Return a list of
        :class:`~click.shell_completion.CompletionItem` objects for the
        incomplete value. Most types do not provide completions, but
        some do, and this allows custom types to provide custom
        completions as well.

        :param ctx: Invocation context for this command.
        :param param: The parameter that is requesting completion.
        :param incomplete: Value being completed. May be empty.

        .. versionadded:: 8.0
        """
        return []
```
- test_path_surrogates (tests/test_types/test_Path.py:90-143)
```python
def test_path_surrogates(tmp_path, monkeypatch):
    monkeypatch.chdir(tmp_path)
    type = click.Path(exists=True)
    path = pathlib.Path("\udcff")

    with pytest.raises(click.BadParameter, match="'�' does not exist"):
        type.convert(path, None, None)

    type = click.Path(file_okay=False)
    path.touch()

    with pytest.raises(click.BadParameter, match="'�' is a file"):
        type.convert(path, None, None)

    path.unlink()
    type = click.Path(dir_okay=False)
    path.mkdir()

    with pytest.raises(click.BadParameter, match="'�' is a directory"):
        type.convert(path, None, None)

    path.rmdir()

    def no_access(*args, **kwargs):
        """Test environments may be running as root, so we have to fake the result of
        the access tests that use os.access
        """
        p = args[0]
        assert p == path, f"unexpected os.access call on file not under test: {p!r}"
        return False

    path.touch()
    type = click.Path(readable=True)

    with pytest.raises(click.BadParameter, match="'�' is not readable"):
        with monkeypatch.context() as m:
            m.setattr(os, "access", no_access)
            type.convert(path, None, None)

    type = click.Path(readable=False, writable=True)

    with pytest.raises(click.BadParameter, match="'�' is not writable"):
        with monkeypatch.context() as m:
            m.setattr(os, "access", no_access)
            type.convert(path, None, None)

    type = click.Path(readable=False, executable=True)

    with pytest.raises(click.BadParameter, match="'�' is not executable"):
        with monkeypatch.context() as m:
            m.setattr(os, "access", no_access)
            type.convert(path, None, None)

    path.unlink()
```
- prompt (src/click/termui.py:168-286)
```python
def prompt(
    text: str,
    default: V | str | None = None,
    hide_input: bool = False,
    confirmation_prompt: bool | str = False,
    type: ParamType[V, str] | type[V] | None = None,
    value_proc: t.Callable[[str], V] | None = None,
    prompt_suffix: str = ": ",
    show_default: bool | str = True,
    err: bool = False,
    show_choices: bool = True,
) -> V:
    """Prompts a user for input.  This is a convenience function that can
    be used to prompt a user for input later.

    If the user aborts the input by sending an interrupt signal, this
    function will catch it and raise a :exc:`Abort` exception.

    :param text: the text to show for the prompt.
    :param default: the default value to use if no input happens.  If this
                    is not given it will prompt until it's aborted.
    :param hide_input: if this is set to true then the input value will
                       be hidden.
    :param confirmation_prompt: Prompt a second time to confirm the
        value. Can be set to a string instead of ``True`` to customize
        the message.
    :param type: the type to use to check the value against.
    :param value_proc: if this parameter is provided it's a function that
                       is invoked instead of the type conversion to
                       convert a value.
    :param prompt_suffix: a suffix that should be added to the prompt.
    :param show_default: shows or hides the default value in the prompt.
                         If this value is a string, it shows that string
                         in parentheses instead of the actual value.
    :param err: if set to true the file defaults to ``stderr`` instead of
                ``stdout``, the same as with echo.
    :param show_choices: Show or hide choices if the passed type is a Choice.
                         For example if type is a Choice of either day or week,
                         show_choices is true and text is "Group by" then the
                         prompt will be "Group by (day, week): ".

    .. versionchanged:: 8.5.0
        Generically typed: the return type is narrowed by ``type``,
        ``value_proc``, or ``default`` instead of being ``Any``. Runtime
        behavior is unchanged.

    .. versionchanged:: 8.3.3
        ``show_default`` can be a string to show a custom value instead
        of the actual default, matching the help text behavior.

    .. versionchanged:: 8.3.1
        A space is no longer appended to the prompt.

    .. versionadded:: 8.0
        ``confirmation_prompt`` can be a custom string.

    .. versionadded:: 7.0
        Added the ``show_choices`` parameter.

    .. versionadded:: 6.0
        Added unicode support for cmd.exe on Windows.

    .. versionadded:: 4.0
        Added the `err` parameter.

    """

    def prompt_func(text: str) -> str:
        f = hidden_prompt_func if hide_input else visible_prompt_func
        try:
            return _readline_prompt(f, text, err)
        except (KeyboardInterrupt, EOFError):
            # getpass doesn't print a newline if the user aborts input with ^C.
            # Allegedly this behavior is inherited from getpass(3).
            # A doc bug has been filed at https://bugs.python.org/issue24711
            if hide_input:
                echo(None, err=err)
            raise Abort() from None

    if value_proc is None:
        value_proc = convert_type(type, default)

    prompt = _build_prompt(
        text, prompt_suffix, show_default, default, show_choices, type
    )

    if confirmation_prompt:
        if confirmation_prompt is True:
            confirmation_prompt = _("Repeat for confirmation")

        confirmation_prompt = _build_prompt(confirmation_prompt, prompt_suffix)

    while True:
        while True:
            value = prompt_func(prompt)
            if value:
                break
            elif default is not None:
                # Defaults of any type are accepted and round trip through
                # value_proc like typed input, so the annotation is only
                # accurate for typed input.
                value = t.cast("str", default)
                break
        try:
            result = value_proc(value)
        except UsageError as e:
            message = _mask_hidden_input(e.message, value) if hide_input else e.message
            echo(_("Error: {message}").format(message=message), err=err)
            continue
        if not confirmation_prompt:
            return result
        while True:
            value2 = prompt_func(confirmation_prompt)
            is_empty = not value and not value2
            if value2 or is_empty:
                break
        if value == value2:
            return result
        echo(_("Error: The two entered values do not match."), err=err)
```
- DateTime (src/click/types.py:511-579)
```python
class DateTime(ParamType[datetime]):
    """The DateTime type converts date strings into `datetime` objects.

    The format strings which are checked are configurable, but default to some
    common (non-timezone aware) ISO 8601 formats.

    When specifying *DateTime* formats, you should only pass a list or a tuple.
    Other iterables, like generators, may lead to surprising results.

    The format strings are processed using ``datetime.strptime``, and this
    consequently defines the format strings which are allowed.

    Parsing is tried using each format, in order, and the first format which
    parses successfully is used.

    :param formats: A list or tuple of date format strings, in the order in
