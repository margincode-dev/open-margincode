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
