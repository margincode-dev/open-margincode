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
