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
            # value set and is setup to match at least one value.
            if attrs.get("default", UNSET) is UNSET:
                required = attrs.get("nargs", 1) > 0
            # If the argument has a default value, it is not required.
            else:
                required = False

        if "multiple" in attrs:
            raise TypeError("__init__() got an unexpected keyword argument 'multiple'.")

        super().__init__(param_decls, required=required, help=help, **attrs)

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
                self._check_name_is_usable("", decls)
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
        self._check_name_is_usable(name, decls)
        return name, [arg], []

    def get_usage_pieces(self, ctx: Context) -> list[str]:
        return [self.make_metavar(ctx)]

    def get_help_record(self, ctx: Context) -> tuple[str, str]:
        """Returns the argument's help row: its metavar and its help text.

        Unlike :meth:`Option.get_help_record`, this never returns ``None``. An
        argument cannot be hidden, so an undocumented one still gets a row, with
        an empty description.

        .. versionchanged:: 8.5.1
            Always returns a tuple. It used to return ``None`` when ``help`` was
            not set.
        """
        return self.make_metavar(ctx), self.help or ""

    def get_error_hint(self, ctx: Context | None) -> str:
        if ctx is not None:
            return f"'{self.make_metavar(ctx)}'"
        return f"'{self.human_readable_name}'"

    def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
        parser.add_argument(dest=self.name, nargs=self.nargs, obj=self)
```
- Parameter.handle_parse_result (src/click/core.py:2835-2921)
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
- Path (src/click/types.py:1059-1251)
```python
class Path(ParamType[_PathT_co, str | os.PathLike[str]], t.Generic[_PathT_co]):
    """The ``Path`` type is similar to the :class:`File` type, but
    returns the filename instead of an open file. Various checks can be
    enabled to validate the type of file and permissions.

    :param exists: The file or directory needs to exist for the value to
        be valid. If this is not set to ``True``, and the file does not
        exist, then all further checks are silently skipped.
    :param file_okay: Allow a file as a value.
    :param dir_okay: Allow a directory as a value.
    :param readable: if true, a readable check is performed.
    :param writable: if true, a writable check is performed.
    :param executable: if true, an executable check is performed.
    :param resolve_path: Make the value absolute and resolve any
        symlinks. A ``~`` is not expanded, as this is supposed to be
        done by the shell only.
    :param allow_dash: Allow a single dash as a value, which indicates
        a standard stream (but does not open it). Use
        :func:`~click.open_file` to handle opening this value.
    :param path_type: Convert the incoming path value to this type. If
        ``None``, keep Python's default, which is ``str``. Useful to
        convert to :class:`pathlib.Path`.

    .. versionchanged:: 8.5.1
        Reveal the actual type of the value as converted from raw input.

    .. versionchanged:: 8.1
        Added the ``executable`` parameter.

    .. versionchanged:: 8.0
        Allow passing ``path_type=pathlib.Path``.

    .. versionchanged:: 6.0
        Added the ``allow_dash`` parameter.
    """

    envvar_list_splitter: t.ClassVar[str] = os.path.pathsep

    exists: bool
    file_okay: bool
    dir_okay: bool
    readable: bool
    writable: bool
    executable: bool
    resolve_path: bool
    allow_dash: bool
    name: str

    def __init__(
        self,
        exists: bool = False,
        file_okay: bool = True,
        dir_okay: bool = True,
        writable: bool = False,
        readable: bool = True,
        resolve_path: bool = False,
        allow_dash: bool = False,
        path_type: type[_PathT_co] | None = None,
        executable: bool = False,
    ) -> None:
        self.exists = exists
        self.file_okay = file_okay
        self.dir_okay = dir_okay
        self.readable = readable
        self.writable = writable
        self.executable = executable
        self.resolve_path = resolve_path
        self.allow_dash = allow_dash
        self.type: type[_PathT_co] | None = path_type

        if self.file_okay and not self.dir_okay:
            self.name = _("file")
        elif self.dir_okay and not self.file_okay:
            self.name = _("directory")
        else:
            self.name = _("path")

    def to_info_dict(self) -> PathInfoDict:
        return {
            "exists": self.exists,
            "file_okay": self.file_okay,
            "dir_okay": self.dir_okay,
            "writable": self.writable,
            "readable": self.readable,
            "allow_dash": self.allow_dash,
            **super().to_info_dict(),
        }

    def coerce_path_result(self, value: str | os.PathLike[str]) -> _PathT_co:
        path_type: type[t.Any] | None = self.type
        rv: t.Any = value

        if path_type is not None and not isinstance(value, path_type):
            if path_type is str:
                rv = os.fsdecode(value)
            elif path_type is bytes:
                rv = os.fsencode(value)
            else:
                rv = path_type(value)

        return t.cast("_PathT_co", rv)

    def convert(
        self,
        value: str | os.PathLike[str],
        param: Parameter | None,
        ctx: Context | None,
    ) -> _PathT_co:
        rv = value

        dash = b"-" if isinstance(rv, bytes) else "-"
        is_dash = self.file_okay and self.allow_dash and rv == dash

        if not is_dash:
            if self.resolve_path:
                rv = os.path.realpath(rv)

            try:
                st = os.stat(rv)
            except OSError:
                if not self.exists:
                    return self.coerce_path_result(rv)
                self.fail(
                    _("{name} {filename!r} does not exist.").format(
                        name=self.name.title(), filename=format_filename(value)
                    ),
                    param,
                    ctx,
                )

            if not self.file_okay and stat.S_ISREG(st.st_mode):
                self.fail(
                    _("{name} {filename!r} is a file.").format(
                        name=self.name.title(), filename=format_filename(value)
                    ),
                    param,
                    ctx,
                )
            if not self.dir_okay and stat.S_ISDIR(st.st_mode):
                self.fail(
                    _("{name} {filename!r} is a directory.").format(
                        name=self.name.title(), filename=format_filename(value)
                    ),
                    param,
                    ctx,
                )

            if self.readable and not os.access(rv, os.R_OK):
                self.fail(
                    _("{name} {filename!r} is not readable.").format(
                        name=self.name.title(), filename=format_filename(value)
                    ),
                    param,
                    ctx,
                )

            if self.writable and not os.access(rv, os.W_OK):
                self.fail(
                    _("{name} {filename!r} is not writable.").format(
                        name=self.name.title(), filename=format_filename(value)
                    ),
                    param,
                    ctx,
                )

            if self.executable and not os.access(value, os.X_OK):
                self.fail(
                    _("{name} {filename!r} is not executable.").format(
                        name=self.name.title(), filename=format_filename(value)
                    ),
                    param,
                    ctx,
                )

        return self.coerce_path_result(rv)

    def shell_complete(
        self, ctx: Context, param: Parameter, incomplete: str
    ) -> list[CompletionItem]:
        """Return a special completion marker that tells the completion
        system to use the shell to provide path completions for only
        directories or any paths.

        :param ctx: Invocation context for this command.
        :param param: The parameter that is requesting completion.
        :param incomplete: Value being completed. May be empty.

        .. versionadded:: 8.0
        """
        from click.shell_completion import CompletionItem

        type = "dir" if self.dir_okay and not self.file_okay else "file"
        return [CompletionItem(incomplete, type=type)]
```
- test_path_surrogates (tests/test_types/test_Path.py:91-144)
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
- _unpack_args (src/click/parser.py:51-108)
```python
def _unpack_args(
    args: cabc.Sequence[str], nargs_spec: cabc.Sequence[int]
) -> tuple[cabc.Sequence[str | cabc.Sequence[str | T_UNSET] | T_UNSET], list[str]]:
    """Given an iterable of arguments and an iterable of nargs specifications,
    it returns a tuple with all the unpacked arguments at the first index
    and all remaining arguments as the second.

    The nargs specification is the number of arguments that should be consumed
    or `-1` to indicate that this position should eat up all the remainders.

    Missing items are filled with ``UNSET``.
    """
    args = deque(args)
    nargs_spec = deque(nargs_spec)
    rv: list[str | tuple[str | T_UNSET, ...] | T_UNSET] = []
    spos: int | None = None

    def _fetch(c: deque[str]) -> str | T_UNSET:
        try:
            if spos is None:
                return c.popleft()
            else:
                return c.pop()
        except IndexError:
            return UNSET

    while nargs_spec:
        if spos is None:
            nargs = nargs_spec.popleft()
        else:
            nargs = nargs_spec.pop()

        if nargs == 1:
            rv.append(_fetch(args))
        elif nargs > 1:
            x: list[str | T_UNSET] = [_fetch(args) for _ in range(nargs)]

            # If we're reversed, we're pulling in the arguments in reverse,
            # so we need to turn them around.
            if spos is not None:
                x.reverse()

            rv.append(tuple(x))
        elif nargs < 0:
            if spos is not None:
                raise TypeError("Cannot have two nargs < 0")

            spos = len(rv)
            rv.append(UNSET)

    # spos is the position of the wildcard (star).  If it's not `None`,
    # we fill it with the remainder.
    if spos is not None:
        rv[spos] = tuple(args)
        args = []
        rv[spos + 1 :] = reversed(rv[spos + 1 :])

    return tuple(rv), list(args)
```
- Parameter.__init__ (src/click/core.py:2356-2438)
```python
    def __init__(
        self,
        param_decls: cabc.Sequence[str] | None = None,
        type: types.ParamType[t.Any] | t.Any | None = None,
        required: bool = False,
        # XXX The default historically embed two concepts:
        # - the declaration of a Parameter object carrying the default (handy to
        #   arbitrage the default value of coupled Parameters sharing the same
        #   self.name, like flag options),
        # - and the actual value of the default.
        # It is confusing and is the source of many issues discussed in:
        # https://github.com/pallets/click/pull/3030
        # In the future, we might think of splitting it in two, not unlike
        # Option.is_flag and Option.flag_value: we could have something like
        # Parameter.is_default and Parameter.default_value.
        default: t.Any | t.Callable[[], t.Any] | None = UNSET,
        callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None,
        nargs: int | None = None,
        multiple: bool = False,
        metavar: str | None = None,
        expose_value: bool = True,
        is_eager: bool = False,
        envvar: str | cabc.Sequence[str] | None = None,
        shell_complete: t.Callable[
            [Context, Parameter, str], list[CompletionItem] | list[str]
        ]
        | None = None,
        deprecated: bool | str = False,
        help: str | None = None,
    ) -> None:
        self.name, self.opts, self.secondary_opts = self._parse_decls(
            param_decls or (), expose_value
        )
        self.type: types.ParamType[t.Any] = types.convert_type(type, default)

        # Default nargs to what the type tells us if we have that
        # information available.
        if nargs is None:
            if self.type.is_composite:
                nargs = self.type.arity
            else:
                nargs = 1

        self.required = required
        self.callback = callback
        self.nargs = nargs
        self.multiple = multiple
        self.expose_value = expose_value
        self.default = default
        # Whether the user passed ``default`` explicitly to the constructor.
        # Captured before any auto-derived default (like ``False`` for boolean
        # flags in :class:`Option`) replaces the :data:`UNSET` sentinel, so it
        # remains ``False`` when the default was inferred rather than chosen.
        # Refs: https://github.com/pallets/click/issues/3403
        self._default_explicit = default is not UNSET
        self.is_eager = is_eager
        self.metavar = metavar
        self.envvar = envvar
        self._custom_shell_complete = shell_complete
        self.deprecated = deprecated

        if help:
            help = inspect.cleandoc(help)

        if deprecated:
            label = _format_deprecated_label(deprecated)
            help = f"{help} {label}" if help else label

        self.help = help

        if __debug__:
            if self.type.is_composite and nargs != self.type.arity:
                raise ValueError(
                    f"'nargs' must be {self.type.arity} (or None) for"
                    f" type {self.type!r}, but it was {nargs}."
                )

            if required and deprecated:
                raise ValueError(
                    f"The {self.param_type_name} '{self.human_readable_name}' "
                    "is deprecated and still required. A deprecated "
                    f"{self.param_type_name} cannot be required."
                )
```
- Option.get_help_spec (src/click/core.py:3464-3493)
```python
    def get_help_spec(self, ctx: Context) -> str:
        """Returns the left column of the option's help record: its spellings
        and metavar, like ``-v, --verbose`` or ``-c, --config TEXT``.

        Unlike :meth:`get_help_record`, the spec is produced even when the
        option is :attr:`hidden`.

        .. versionadded:: 8.5.1
        """
        any_prefix_is_slash = False

        def _write_opts(opts: cabc.Sequence[str]) -> str:
            nonlocal any_prefix_is_slash

            rv, any_slashes = join_options(opts)

            if any_slashes:
                any_prefix_is_slash = True

            if not self.is_flag and not self.count:
                rv += f" {self.make_metavar(ctx=ctx)}"

            return rv

        rv = [_write_opts(self.opts)]

        if self.secondary_opts:
            rv.append(_write_opts(self.secondary_opts))

        return ("; " if any_prefix_is_slash else " / ").join(rv)
```
- _FixupStream (src/click/_compat.py:85-151)
```python
class _FixupStream:
    """The new io interface needs more from streams than streams
    traditionally implement.  As such, this fix-up code is necessary in
    some circumstances.

    The forcing of readable and writable flags are there because some tools
    put badly patched objects on sys (one such offender are certain version
    of jupyter notebook).
    """

    def __init__(
        self,
        stream: t.BinaryIO,
        force_readable: bool = False,
        force_writable: bool = False,
    ):
        self._stream = stream
        self._force_readable = force_readable
        self._force_writable = force_writable

    def __getattr__(self, name: str) -> t.Any:
        return getattr(self._stream, name)

    def read1(self, size: int) -> bytes:
        f = getattr(self._stream, "read1", None)

        if f is not None:
            return t.cast(bytes, f(size))

        return self._stream.read(size)

    def readable(self) -> bool:
        if self._force_readable:
            return True
        x = getattr(self._stream, "readable", None)
        if x is not None:
            return t.cast(bool, x())
        try:
            self._stream.read(0)
        except Exception:
            return False
        return True

    def writable(self) -> bool:
        if self._force_writable:
            return True
        x = getattr(self._stream, "writable", None)
        if x is not None:
            return t.cast(bool, x())
        try:
            self._stream.write(b"")
        except Exception:
            try:
                self._stream.write(b"")
            except Exception:
                return False
        return True

    def seekable(self) -> bool:
        x = getattr(self._stream, "seekable", None)
        if x is not None:
            return t.cast(bool, x())
        try:
            self._stream.seek(self._stream.tell())
        except Exception:
            return False
        return True
```
- Parameter (src/click/core.py:2237-2960)
```python
class Parameter(ABC):
    r"""A parameter to a command comes in two versions: they are either
    :class:`Option`\s or :class:`Argument`\s.  Other subclasses are currently
    not supported by design as some of the internals for parsing are
    intentionally not finalized.

    Some settings are supported by both options and arguments.

    :param param_decls: the parameter declarations for this option or
                        argument.  This is a list of flags or argument
                        names.
    :param type: the type that should be used.  Either a :class:`ParamType`
                 or a Python type.  The latter is converted into the former
                 automatically if supported.
    :param required: controls if this is optional or not.
    :param default: the default value if omitted.  This can also be a callable,
                    in which case it's invoked when the default is needed
                    without any arguments.
    :param callback: A function to further process or validate the value
        after type conversion. It is called as ``f(ctx, param, value)``
        and must return the value. It is called for all sources,
        including prompts.
    :param nargs: the number of arguments to match.  If not ``1`` the return
                  value is a tuple instead of single value.  The default for
                  nargs is ``1`` (except if the type is a tuple, then it's
                  the arity of the tuple). If ``nargs=-1``, all remaining
                  parameters are collected.
    :param metavar: how the value is represented in the help page.
    :param expose_value: if this is `True` then the value is passed onwards
                         to the command callback and stored on the context,
                         otherwise it's skipped.
    :param is_eager: eager values are processed before non eager ones.  This
                     should not be set for arguments or it will inverse the
                     order of processing.
    :param envvar: environment variable(s) that are used to provide a default value for
        this parameter. This can be a string or a sequence of strings. If a sequence is
        given, only the first non-empty environment variable is used for the parameter.
    :param shell_complete: A function that returns custom shell
        completions. Used instead of the param's type completion if
        given. Takes ``ctx, param, incomplete`` and must return a list
        of :class:`~click.shell_completion.CompletionItem` or a list of
        strings.
    :param deprecated: If ``True`` or non-empty string, issues a message
                        indicating that the argument is deprecated and highlights
                        its deprecation in --help. The message can be customized
                        by using a string as the value. A deprecated parameter
                        cannot be required, a ValueError will be raised otherwise.
    :param help: the help string. It is dedented and get a deprecated label if
        appropriate.

    .. versionchanged:: 8.5.1
        New ``help`` parameter to replace the one from :class:`Option` and
        :class:`Argument`.

    .. versionchanged:: 8.2.0
        Introduction of ``deprecated``.

    .. versionchanged:: 8.2
        Adding duplicate parameter names to a :class:`~click.core.Command` will
        result in a ``UserWarning`` being shown.

    .. versionchanged:: 8.2
        Adding duplicate parameter names to a :class:`~click.core.Command` will
        result in a ``UserWarning`` being shown.

    .. versionchanged:: 8.0
        ``process_value`` validates required parameters and bounded
        ``nargs``, and invokes the parameter callback before returning
        the value. This allows the callback to validate prompts.
        ``full_process_value`` is removed.

    .. versionchanged:: 8.0
        ``autocompletion`` is renamed to ``shell_complete`` and has new
        semantics described above. The old name is deprecated and will
        be removed in 8.1, until then it will be wrapped to match the
        new requirements.

    .. versionchanged:: 8.0
        For ``multiple=True, nargs>1``, the default must be a list of
        tuples.

    .. versionchanged:: 8.0
        Setting a default is no longer required for ``nargs>1``, it will
        default to ``None``. ``multiple=True`` or ``nargs=-1`` will
        default to ``()``.

    .. versionchanged:: 7.1
        Empty environment variables are ignored rather than taking the
        empty string value. This makes it possible for scripts to clear
        variables if they can't unset them.

    .. versionchanged:: 2.0
        Changed signature for parameter callback to also be passed the
        parameter. The old callback format will still work, but it will
        raise a warning to give you a chance to migrate the code easier.
    """

    param_type_name = "parameter"

    name: str
    opts: list[str]
    secondary_opts: list[str]
    # `Parameter.type` is annotated in `__init__` to avoid confusing mypy
    required: bool
    callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None
    nargs: int
    multiple: bool
    expose_value: bool
    default: t.Any | t.Callable[[], t.Any] | None
    _default_explicit: bool
    is_eager: bool
    metavar: str | None
    envvar: str | cabc.Sequence[str] | None
    _custom_shell_complete: (
        t.Callable[[Context, Parameter, str], list[CompletionItem] | list[str]] | None
    )
    deprecated: bool | str
    help: str | None

    def __init__(
        self,
        param_decls: cabc.Sequence[str] | None = None,
        type: types.ParamType[t.Any] | t.Any | None = None,
        required: bool = False,
        # XXX The default historically embed two concepts:
        # - the declaration of a Parameter object carrying the default (handy to
        #   arbitrage the default value of coupled Parameters sharing the same
        #   self.name, like flag options),
        # - and the actual value of the default.
        # It is confusing and is the source of many issues discussed in:
        # https://github.com/pallets/click/pull/3030
        # In the future, we might think of splitting it in two, not unlike
        # Option.is_flag and Option.flag_value: we could have something like
        # Parameter.is_default and Parameter.default_value.
        default: t.Any | t.Callable[[], t.Any] | None = UNSET,
        callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None,
        nargs: int | None = None,
        multiple: bool = False,
        metavar: str | None = None,
        expose_value: bool = True,
        is_eager: bool = False,
        envvar: str | cabc.Sequence[str] | None = None,
        shell_complete: t.Callable[
            [Context, Parameter, str], list[CompletionItem] | list[str]
        ]
        | None = None,
        deprecated: bool | str = False,
        help: str | None = None,
    ) -> None:
        self.name, self.opts, self.secondary_opts = self._parse_decls(
            param_decls or (), expose_value
        )
        self.type: types.ParamType[t.Any] = types.convert_type(type, default)

        # Default nargs to what the type tells us if we have that
        # information available.
        if nargs is None:
            if self.type.is_composite:
                nargs = self.type.arity
            else:
                nargs = 1

        self.required = required
        self.callback = callback
        self.nargs = nargs
        self.multiple = multiple
        self.expose_value = expose_value
        self.default = default
