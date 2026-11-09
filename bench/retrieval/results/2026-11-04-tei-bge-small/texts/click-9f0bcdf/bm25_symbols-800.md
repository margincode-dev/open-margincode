- Parameter.human_readable_name (src/click/core.py:2520-2524)
```python
    def human_readable_name(self) -> str:
        """Returns the human readable name of this parameter.  This is the
        same as the name for options, but the metavar for arguments.
        """
        return self.name
```
- Argument.human_readable_name (src/click/core.py:3838-3841)
```python
    def human_readable_name(self) -> str:
        if self.metavar is not None:
            return self.metavar
        return self.name.upper()
```
- Parameter.get_error_hint (src/click/core.py:2929-2937)
```python
    def get_error_hint(self, ctx: Context | None) -> str:
        """Get a stringified version of the param for use in error messages to
        indicate which param caused the error.

        .. versionchanged:: 8.4.0
            ``ctx`` can be ``None``.
        """
        hint_list = self.opts or [self.human_readable_name]
        return " / ".join(f"'{x}'" for x in hint_list)
```
- Argument.get_error_hint (src/click/core.py:3899-3902)
```python
    def get_error_hint(self, ctx: Context | None) -> str:
        if ctx is not None:
            return f"'{self.make_metavar(ctx)}'"
        return f"'{self.human_readable_name}'"
```
- test_parameter_name_not_an_identifier_is_deprecated (tests/test_arguments.py:278-293)
```python
def test_parameter_name_not_an_identifier_is_deprecated(cls, form, decl):
    """Click 9.0 refuses every one of these; 8.6 keeps them and says so.

    An exposed option is the exception, since it already refuses them.
    """
    spec = form.format(decl=decl)

    if cls is click.Option:
        with pytest.raises(TypeError, match="Could not determine name"):
            cls([spec])
        return

    with pytest.warns(DeprecationWarning, match="not a valid Python identifier"):
        param = cls([spec])

    assert not param.name.isidentifier()
```
- test_help_spec (tests/test_options.py:87-98)
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
- Argument (src/click/core.py:3800-3905)
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
