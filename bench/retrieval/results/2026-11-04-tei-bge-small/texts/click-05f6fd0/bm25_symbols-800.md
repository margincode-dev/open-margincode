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
