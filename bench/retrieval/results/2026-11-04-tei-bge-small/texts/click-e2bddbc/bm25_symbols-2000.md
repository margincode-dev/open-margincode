- test_unset_in_default_map (tests/test_defaults.py:551-567)
```python
def test_unset_in_default_map(runner):
    """An ``UNSET`` value in ``default_map`` should be treated as if
    the key is absent, and so fallback to the parameter's own default.

    Refs: https://github.com/pallets/click/pull/3224#issuecomment-3968643305
    """

    @click.command(
        context_settings={"default_map": {"port": UNSET}},
    )
    @click.option("--port", default=8000)
    def cli(port):
        click.echo(f"port={port}")

    result = runner.invoke(cli, [])
    assert result.exit_code == 0
    assert result.output.strip() == "port=8000"
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
- test_expand_args (tests/test_utils/test__expand_args.py:6-17)
```python
def test_expand_args(monkeypatch):
    user = os.path.expanduser("~")
    assert user in click.utils._expand_args(["~"])
    monkeypatch.setenv("CLICK_TEST", "hello")
    assert "hello" in click.utils._expand_args(["$CLICK_TEST"])
    assert "pyproject.toml" in click.utils._expand_args(["*.toml"])
    assert os.path.join("tests", "conftest.py") in click.utils._expand_args(
        ["**/conftest.py"]
    )
    assert "*.not-found" in click.utils._expand_args(["*.not-found"])
    # a bad glob pattern, such as a pytest identifier, should return itself
    assert click.utils._expand_args(["test.py::test_bad"])[0] == "test.py::test_bad"
```
- _resolve_pager_command (src/click/_termui_impl.py:423-448)
```python
def _resolve_pager_command(cmd_parts: list[str]) -> tuple[Path, list[str]] | None:
    """Resolve a pager ``argv`` to an absolute command path and its parameters.

    The command is looked up with :func:`shutil.which`, which the
    :mod:`subprocess` docs recommend on every platform for an unqualified name:
    https://docs.python.org/3/library/subprocess.html#popen-constructor

    Returns ``None`` when there is no command to run or it is not on the path,
    leaving the caller to pick a fallback.
    """
    if not cmd_parts:
        return None

    import shutil

    cmd_filepath = shutil.which(cmd_parts[0])

    if not cmd_filepath:
        return None

    # Normalize to an absolute path without resolving symlinks: multi-call
    # binaries such as busybox derive their identity from the link they are
    # invoked through, so resolving less -> busybox makes them misbehave.
    # See https://github.com/pallets/click/issues/2943 and
    # https://github.com/pallets/click/pull/2944
    return Path(cmd_filepath).absolute(), cmd_parts[1:]
```
- __getattr__ (src/click/__init__.py:76-144)
```python
def __getattr__(name: str) -> object:
    import warnings

    if name == "BaseCommand":
        from .core import _BaseCommand

        warnings.warn(
            "'BaseCommand' is deprecated and will be removed in Click 9.0. Use"
            " 'Command' instead.",
            DeprecationWarning,
            stacklevel=2,
        )
        return _BaseCommand

    if name == "MultiCommand":
        from .core import _MultiCommand

        warnings.warn(
            "'MultiCommand' is deprecated and will be removed in Click 9.0. Use"
            " 'Group' instead.",
            DeprecationWarning,
            stacklevel=2,
        )
        return _MultiCommand

    if name == "OptionParser":
        from .parser import _OptionParser

        warnings.warn(
            "'OptionParser' is deprecated and will be removed in Click 9.0. The"
            " old parser is available in 'optparse'.",
            DeprecationWarning,
            stacklevel=2,
        )
        return _OptionParser

    if name == "__version__":
        import importlib.metadata

        warnings.warn(
            "The '__version__' attribute is deprecated and will be removed in"
            " Click 9.1. Use feature detection or"
            " 'importlib.metadata.version(\"click\")' instead.",
            DeprecationWarning,
            stacklevel=2,
        )
        return importlib.metadata.version("click")

    if name == "get_binary_stream":
        from .utils import _get_binary_stream

        warnings.warn(
            "'get_binary_stream' is deprecated and will be removed in Click 9.0.",
            DeprecationWarning,
            stacklevel=2,
        )
        return _get_binary_stream

    if name == "get_text_stream":
        from .utils import _get_text_stream

        warnings.warn(
            "'get_text_stream' is deprecated and will be removed in Click 9.0.",
            DeprecationWarning,
            stacklevel=2,
        )
        return _get_text_stream

    raise AttributeError(name)
```
- __getattr__ (src/click/core.py:3814-3835)
```python
def __getattr__(name: str) -> object:
    import warnings

    if name == "BaseCommand":
        warnings.warn(
            "'BaseCommand' is deprecated and will be removed in Click 9.0. Use"
            " 'Command' instead.",
            DeprecationWarning,
            stacklevel=2,
        )
        return _BaseCommand

    if name == "MultiCommand":
        warnings.warn(
            "'MultiCommand' is deprecated and will be removed in Click 9.0. Use"
            " 'Group' instead.",
            DeprecationWarning,
            stacklevel=2,
        )
        return _MultiCommand

    raise AttributeError(name)
```
- __getattr__ (src/click/parser.py:503-533)
```python
def __getattr__(name: str) -> object:
    import warnings

    if name in {
        "OptionParser",
        "Argument",
        "Option",
        "split_opt",
        "normalize_opt",
        "ParsingState",
    }:
        warnings.warn(
            f"'parser.{name}' is deprecated and will be removed in Click 9.0."
            " The old parser is available in 'optparse'.",
            DeprecationWarning,
            stacklevel=2,
        )
        return globals()[f"_{name}"]

    if name == "split_arg_string":
        from .shell_completion import split_arg_string

        warnings.warn(
            "Importing 'parser.split_arg_string' is deprecated, it will only be"
            " available in 'shell_completion' in Click 9.0.",
            DeprecationWarning,
            stacklevel=2,
        )
        return split_arg_string

    raise AttributeError(name)
```
- _tempfilepager (src/click/_termui_impl.py:636-667)
```python
def _tempfilepager(
    cmd_path: Path, cmd_params: list[str], color: bool | None = None
) -> t.Iterator[tuple[t.TextIO, bool | None]]:
    """Page through text by invoking a program on a temporary file.

    Used as the primary pager strategy on Windows (where piping to
    ``more`` adds spurious ``\\r\\n``), and as a fallback on other
