## Task
# click-9f0bcdf · click-9f0bcdf · open
Goal: Add `Parameter.spec` and deprecate `human_readable_name`
Workspace: /root/bench/ws/rebf815-click-9f0bcdf/click-9f0bcdf
Changed (0): nothing yet

## Map
- src/click/core.py
- src/click/_compat.py
- src/click/_winconsole.py
  L53: STDIN_HANDLE = GetStdHandle(-10)
  L54: STDOUT_HANDLE = GetStdHandle(-11)
  L55: STDERR_HANDLE = GetStdHandle(-12)
  L60: ERROR_SUCCESS = 0
  L61: ERROR_NOT_ENOUGH_MEMORY = 8
  L62: ERROR_OPERATION_ABORTED = 995
  L64: EOF = b"\x1a"
  L65: MAX_BYTES_WRITTEN = 32767
  L114: class _WindowsConsoleRawIOBase(io.RawIOBase):
  L115: def __init__(self, handle: int | None) -> None:
  L118: def isatty(self) -> t.Literal[True]:
  L123: class _WindowsConsoleReader(_WindowsConsoleRawIOBase):
  L124: def readable(self) -> t.Literal[True]:
  L127: def readinto(self, b: Buffer) -> int:
  L158: class _WindowsConsoleWriter(_WindowsConsoleRawIOBase):
  L159: def writable(self) -> t.Literal[True]:
  L163: def _get_error_message(errno: int) -> str:
  L170: def write(self, b: Buffer) -> int:
  L190: class ConsoleStream:
  L191: def __init__(self, text_stream: t.TextIO, byte_stream: t.BinaryIO) -> None:
  L196: def name(self) -> str:
  L199: def write(self, x: t.AnyStr) -> int:
  L208: def writelines(self, lines: cabc.Iterable[t.AnyStr]) -> None:
  L212: def __getattr__(self, name: str) -> t.Any:
  L215: def isatty(self) -> bool:
  L218: def __repr__(self) -> str:
  L222: def _get_text_stdin(buffer_stream: t.BinaryIO) -> t.TextIO:
  L232: def _get_text_stdout(buffer_stream: t.BinaryIO) -> t.TextIO:
  L242: def _get_text_stderr(buffer_stream: t.BinaryIO) -> t.TextIO:
  L259: def _is_console(f: t.TextIO) -> bool:
- src/click/testing.py
- src/click/types.py
- examples/aliases/aliases.py
  L7: class Config:
  L10: def __init__(self):
  L14: def add_alias(self, alias, cmd):
  L17: def read_config(self, filename):
  L25: def write_config(self, filename):
  L37: class AliasedGroup(click.Group):
  L42: def get_command(self, ctx, cmd_name): # Step one: built-in commands as normal
  L70: def resolve_command(self, ctx, args): # always return the command's name, not the alias
  L76: def read_config(ctx, param, value):
  L97: def cli():
  L102: def push():
  L108: def pull():
  L114: def clone():
  L120: def commit():
  L127: def status(config):
  L139: def alias(config, alias_, cmd, config_file):
- examples/complex/complex/cli.py
  L7: CONTEXT_SETTINGS = dict(auto_envvar_prefix="COMPLEX")
  L10: class Environment:
  L11: def __init__(self):
  L15: def log(self, msg, *args):
  L21: def vlog(self, msg, *args):
  L31: class ComplexCLI(click.Group):
  L32: def list_commands(self, ctx):
  L40: def get_command(self, ctx, name):
  L56: def cli(ctx, verbose, home):
- examples/naval/naval.py
  L6: def cli():
  L16: def ship():
  L22: def ship_new(name):
  L32: def ship_move(ship, x, y, speed):
  L41: def ship_shoot(ship, x, y):
  L47: def mine():
  L62: def mine_set(x, y, ty):
  L70: def mine_remove(x, y):
- src/click/__init__.py
  L76: def __getattr__(name: str) -> object:
- src/click/_termui_impl.py
- src/click/_utils.py
- repo map: .devcontainer/ .editorconfig .github/ .gitignore .pre-commit-config.yaml .readthedocs.yaml CHANGES.md LICENSE.txt README.md docs/ examples/ pyproject.toml (178 files)

## Code
- sym:334 Parameter.human_readable_name (src/click/core.py:2520-2524)
  def human_readable_name(self) -> str:
- sym:375 Argument.human_readable_name (src/click/core.py:3838-3841)
  def human_readable_name(self) -> str:
- sym:349 Parameter.get_error_hint (src/click/core.py:2929-2937)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:365 Option.get_help_spec (src/click/core.py:3464-3493)
  def get_help_spec(self, ctx: Context) -> str:
- sym:380 Argument.get_error_hint (src/click/core.py:3899-3902)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:346 Parameter.handle_parse_result (src/click/core.py:2835-2921)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:328 Parameter.__init__ (src/click/core.py:2356-2438)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, help: str | None = None, ) -> None:
- sym:230 _format_deprecated_label (src/click/core.py:128-133)
  def _format_deprecated_label(deprecated: bool | str) -> str:
- sym:373 Argument (src/click/core.py:3800-3905)
  class Argument(Parameter):
- sym:683 Path.convert (src/click/types.py:1161-1233)
  def convert( self, value: str | os.PathLike[str], param: Parameter | None, ctx: Context | None, ) -> _PathT_co:
- sym:608 ParamType.get_metavar (src/click/types.py:169-170)
  def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
- sym:680 Path.__init__ (src/click/types.py:1107-1134)
  def __init__( self, exists: bool = False, file_okay: bool = True, dir_okay: bool = True, writable: bool = False, readable: bool = True, resolve_path: bool = False, allow_dash: bool = False, path_type: type[_PathT_co] | None = None, executable: bool = False, ) -> None:
- sym:465 _unpack_args (src/click/parser.py:51-108)
  def _unpack_args( args: cabc.Sequence[str], nargs_spec: cabc.Sequence[int] ) -> tuple[cabc.Sequence[str | cabc.Sequence[str | T_UNSET] | T_UNSET], list[str]]:
- sym:362 Option._check_name_is_normalized (src/click/core.py:3335-3353)
  def _check_name_is_normalized(self, name: str, decls: cabc.Sequence[str]) -> None:
- sym:641 DateTime.get_metavar (src/click/types.py:557-558)
  def get_metavar(self, param: Parameter, ctx: Context) -> str:
- sym:679 Path (src/click/types.py:1059-1251)
  class Path(ParamType[_PathT_co, str | os.PathLike[str]], t.Generic[_PathT_co]):
- sym:97 _FixupStream (src/click/_compat.py:85-151)
  class _FixupStream:
- sym:98 _FixupStream.__init__ (src/click/_compat.py:95-103)
  def __init__( self, stream: t.BinaryIO, force_readable: bool = False, force_writable: bool = False, ):
- sym:333 Parameter._check_name_is_usable (src/click/core.py:2482-2517)
  def _check_name_is_usable(self, name: str, decls: cabc.Sequence[str]) -> None:
- sym:101 _FixupStream.readable (src/click/_compat.py:116-126)
  def readable(self) -> bool:
- sym:769 test_parameter_name_not_an_identifier_is_deprecated (tests/test_arguments.py:278-293)
  def test_parameter_name_not_an_identifier_is_deprecated(cls, form, decl):
- sym:124 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:214 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:586 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
  def name(self) -> str:
- sym:689 Tuple.name (src/click/types.py:1282-1283)
  def name(self) -> str: # type: ignore[override]
- sym:8 AliasedGroup.resolve_command (examples/aliases/aliases.py:70-73)
  def resolve_command(self, ctx, args): # always return the command's name, not the alias
- sym:32 ComplexCLI.get_command (examples/complex/complex/cli.py:40-45)
  def get_command(self, ctx, name):
- sym:57 ship_new (examples/naval/naval.py:22-24)
  def ship_new(name):
- sym:87 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:91 is_ascii_encoding (src/click/_compat.py:43-48)
  def is_ascii_encoding(encoding: str) -> bool:
- sym:99 _FixupStream.__getattr__ (src/click/_compat.py:105-106)
  def __getattr__(self, name: str) -> t.Any:
- sym:121 open_stream (src/click/_compat.py:374-452)
  def open_stream( filename: str | os.PathLike[str], mode: str = "r", encoding: str | None = None, errors: str | None = "strict", atomic: bool = False, ) -> tuple[t.IO[t.Any], bool]:
- sym:126 _AtomicFile.__getattr__ (src/click/_compat.py:473-474)
  def __getattr__(self, name: str) -> t.Any:
- sym:162 _PagerWriter.__getattr__ (src/click/_termui_impl.py:419-420)
  def __getattr__(self, name: str) -> t.Any:
- sym:168 _tempfilepager (src/click/_termui_impl.py:636-667)
  def _tempfilepager( cmd_path: Path, cmd_params: list[str], color: bool | None = None ) -> t.Iterator[tuple[t.TextIO, bool | None]]:
- sym:186 Sentinel.__repr__ (src/click/_utils.py:18-19)
  def __repr__(self) -> str:
- sym:189 Sentinel.__reduce_ex__ (src/click/_utils.py:27-28)
  def __reduce_ex__(self, protocol: object) -> tuple[t.Any, ...]:
- sym:190 UNSET (src/click/_utils.py:31-31)
  UNSET: t.Literal[Sentinel.UNSET] = Sentinel.UNSET
- sym:217 ConsoleStream.__getattr__ (src/click/_winconsole.py:212-213)
  def __getattr__(self, name: str) -> t.Any:
- sym:219 ConsoleStream.__repr__ (src/click/_winconsole.py:218-219)
  def __repr__(self) -> str:
- sym:226 _complete_visible_commands (src/click/core.py:64-80)
  def _complete_visible_commands( ctx: Context, incomplete: str ) -> cabc.Iterator[tuple[str, Command]]:
- sym:227 _check_nested_chain (src/click/core.py:83-100)
  def _check_nested_chain( base_command: Group, cmd_name: str, cmd: Command, register: bool = False ) -> None:
- sym:231 _format_deprecated_suffix (src/click/core.py:136-142)
  def _format_deprecated_suffix(deprecated: bool | str) -> str:
- sym:233 augment_usage_errors (src/click/core.py:150-165)
  def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
- sym:235 ParameterSource (src/click/core.py:195-231)
  class ParameterSource(enum.IntEnum):
- sym:236 Context (src/click/core.py:234-982)
  class Context:
- sym:253 Context._default_map_has (src/click/core.py:787-799)
  def _default_map_has(self, name: str | None) -> bool:
- sym:254 Context.lookup_default (src/click/core.py:802-804)
- sym:262 Context._make_sub_context (src/click/core.py:867-873)
- sym:267 Context.set_parameter_source (src/click/core.py:957-964)
- sym:268 Context.get_parameter_source (src/click/core.py:966-982)
- sym:270 Command.__init__ (src/click/core.py:1061-1103)
- sym:271 Command.to_info_dict (src/click/core.py:1105-1114)
- sym:272 Command.__repr__ (src/click/core.py:1116-1117)
  def __repr__(self) -> str:
- sym:274 Command.get_params (src/click/core.py:1128-1182)
- sym:277 Command.get_help_option_names (src/click/core.py:1203-1216)
- sym:763 test_parameter_name_is_an_identifier_or_deprecated (tests/test_arguments.py:166-189)
- sym:1014 test_param_to_info_dict_deprecated_help (tests/test_info_dict.py:321-324)
- sym:1025 test_help_spec (tests/test_options.py:87-98)
- sym:1381 test_path_surrogates (tests/test_types/test_Path.py:91-144)
- sym:970 test_unnormalized_explicit_name_deprecated (tests/test_deprecations.py:190-196)
- sym:968 test_usable_name_not_deprecated (tests/test_deprecations.py:161-164)

---
tokens: 3563/4000
