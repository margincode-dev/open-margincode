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
