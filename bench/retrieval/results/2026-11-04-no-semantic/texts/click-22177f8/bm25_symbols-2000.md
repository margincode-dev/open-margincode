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
