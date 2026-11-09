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
    platforms. The command is invoked with any parameters from ``PAGER``
    followed by the temporary file name.
    """
    import subprocess
    import tempfile

    encoding = get_best_encoding(sys.stdout)
    # On Windows, NamedTemporaryFile cannot be opened by another process
    # while Python still has it open, so we use delete=False and clean up manually
    # rather than using a contextmanager here.
    f = tempfile.NamedTemporaryFile(
        mode="w", encoding=encoding, errors="replace", delete=False
    )
    try:
        yield t.cast(t.TextIO, f), color
        f.flush()
        f.close()
        subprocess.call([str(cmd_path), *cmd_params, f.name])
    finally:
        # An error raised while paging skips the close() above, and Windows
        # refuses to unlink a file the process still holds open. Closing here
        # keeps that PermissionError from replacing the original exception.
        # close() is idempotent, so this is a no-op on the success path.
        f.close()
        os.unlink(f.name)
```
- Parameter.handle_parse_result (src/click/core.py:2776-2862)
```python
    def handle_parse_result(
        self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str]
    ) -> tuple[t.Any, list[str]]:
        """Process the value produced by the parser from user input.

        Always process the value through the Parameter's :attr:`type`, wherever it
        comes from.

        If the parameter is deprecated, this method warn the user about it. But only if
        the value has been explicitly set by the user (and as such, is not coming from
        a default).

        :meta private:
        """
        # Capture the slot's existing state before we mutate
        # ``_parameter_source`` so the write decision below can compare our
        # incoming source against the source of the option that already wrote
        # the slot (if any).
        existing_value = ctx.params.get(self.name, UNSET)
        existing_source = ctx.get_parameter_source(self.name)
        existing_default_explicit = ctx._param_default_explicit.get(self.name, False)

        with augment_usage_errors(ctx, param=self):
            value, source = self.consume_value(ctx, opts)

            # Record the source before processing so eager callbacks and type
            # conversion can inspect it. Restored after arbitration if this
            # option loses a feature-switch group.
            ctx.set_parameter_source(self.name, source)

            # Display a deprecation warning if necessary.
            if (
                self.deprecated
                and value is not UNSET
                and source < ParameterSource.DEFAULT_MAP
            ):
                message = _(
                    "DeprecationWarning: The {param_type} {name!r} is deprecated."
                    "{extra_message}"
                ).format(
                    param_type=self.param_type_name,
                    name=self.human_readable_name,
                    extra_message=_format_deprecated_suffix(self.deprecated),
                )
                echo(style(message, fg="red"), err=True)

            # Process the value through the parameter's type.
            try:
                value = self.process_value(ctx, value)
            except Exception:
                if not ctx.resilient_parsing:
                    raise
                # In resilient parsing mode, we do not want to fail the command if the
                # value is incompatible with the parameter type, so we reset the value
                # to UNSET, which will be interpreted as a missing value.
                value = UNSET

        # Arbitrate the slot when several parameters target the same variable
        # name (feature-switch groups). See: https://github.com/pallets/click/issues/3403
        slot_empty = existing_value is UNSET
        more_explicit = existing_source is not None and source < existing_source
        same_source = existing_source is not None and source == existing_source
        auto_would_downgrade_explicit = (
            same_source
            and source == ParameterSource.DEFAULT
            and existing_default_explicit
            and not self._default_explicit
        )
        is_winner = (
            slot_empty
            or more_explicit
            or (same_source and not auto_would_downgrade_explicit)
        )

        if is_winner:
            if self.expose_value:
                ctx.params[self.name] = value
                ctx._param_default_explicit[self.name] = self._default_explicit
        elif existing_source is not None:
            # Lost arbitration; restore the winning option's source.
            ctx.set_parameter_source(self.name, existing_source)
        # else: ctx.params[self.name] was populated by code that bypassed
        # handle_parse_result (from another option's callback for example). Keep
        # the provisional source recorded before process_value so downstream
        # lookups don't return ``None``.

        return value, args
```
- test_param_type_input_parameter_defaults_at_runtime (tests/test_types/test_ParamType.py:6-12)
```python
def test_param_type_input_parameter_defaults_at_runtime():
    """Omitting the input type parameter works at runtime on every
    supported Python. The ``Any`` default is native (PEP 696) on Python
    3.13+, and backfilled by ``ParamType.__class_getitem__`` before
    that."""
    assert t.get_args(click.ParamType[int]) == (int, t.Any)
    assert t.get_args(click.ParamType[int, str]) == (int, str)
```
- test_custom_type_flag_value_dual_options (tests/test_options.py:2395-2417)
```python
def test_custom_type_flag_value_dual_options(
    runner, opt1_params, opt2_params, args, expected
):
    """Test how flag values are processed with dual options competing for the same
    variable name.

    Reproduce issues reported in:
    https://github.com/pallets/click/issues/3024#issuecomment-3146508536
    https://github.com/pallets/click/issues/2012#issue-946471049
    https://github.com/pallets/click/issues/2012#issuecomment-892437060
    """

    @click.command()
    @click.option("--opt1", "dual_option", **opt1_params)
    @click.option("--opt2", "dual_option", **opt2_params)
    def cli(dual_option):
        click.echo(repr(dual_option), nl=False)

    result = runner.invoke(cli, args)
    if isinstance(expected, re.Pattern):
        assert re.match(expected, result.output)
    else:
        assert result.output == repr(expected)
```
- version_option (src/click/decorators.py:421-558)
```python
def version_option(
    version: str | None = None,
    *param_decls: str,
    package_name: str | None = None,
    prog_name: str | None = None,
    message: str | None = None,
    **kwargs: t.Any,
) -> t.Callable[[FC], FC]:
    """Add a ``--version`` option which immediately prints the version
    number and exits the program.

    If ``version`` is not provided, Click will try to detect it using
    :func:`importlib.metadata.version` to get the version for the
    ``package_name``.

    If ``package_name`` is not provided, Click will try to detect it by
    inspecting the stack frames. If the detected (or given) name does
    not match an installed distribution, Click resolves it as an import
    (top-level module) name via
    :func:`importlib.metadata.packages_distributions`, so e.g. ``PIL``
    resolves to the ``Pillow`` distribution.

    .. note::
        The parameters and message variables accepted by this option are
        frozen: no new slots will be added, to keep the common case simple
        and predictable. If you need values it does not expose, such as a
        file path, the Python version, or git metadata, use
        :func:`custom_version_option` to render the output yourself.

        Rationale: `discussion #3527
        <https://github.com/pallets/click/discussions/3527>`_.

    :param version: The version number to show. If not provided, Click
        will try to detect it.
    :param param_decls: One or more option names. Defaults to the single
        value ``"--version"``.
    :param package_name: The package name to detect the version from. If
        not provided, Click will try to detect it.
    :param prog_name: The name of the CLI to show in the message. If not
        provided, it will be detected from the command.
    :param message: The message to show. The values ``%(prog)s``,
        ``%(package)s``, and ``%(version)s`` are available. Defaults to
        ``"%(prog)s, version %(version)s"``.
    :param kwargs: Extra arguments are passed to :func:`option`.
    :raise RuntimeError: ``version`` could not be detected.

    .. versionchanged:: 8.0
        Add the ``package_name`` parameter, and the ``%(package)s``
        value for messages.

    .. versionchanged:: 8.0
        Use :mod:`importlib.metadata` instead of ``pkg_resources``. The
        version is detected based on the package name, not the entry
