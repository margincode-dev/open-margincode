src/click/testing.py:
  L203: def name(self) -> str:
  L382: def get_default_prog_name(self, cli: Command) -> str:
  L32: class EchoingStdin:
  L37: def __init__(self, input: t.BinaryIO, output: t.BinaryIO) -> None:
  L42: def __getattr__(self, x: str) -> t.Any:
  L45: def _echo(self, rv: bytes) -> bytes:
  L51: def read(self, n: int = -1) -> bytes:
  L54: def read1(self, n: int = -1) -> bytes:
  L57: def readline(self, n: int = -1) -> bytes:
  L60: def readlines(self) -> list[bytes]:
  L63: def __iter__(self) -> cabc.Iterator[bytes]:
  L66: def __repr__(self) -> str:
  L71: def _pause_echo(stream: EchoingStdin | None) -> cabc.Generator[None]:
  L80: class _FDCapture:
  L95: def __init__(self, targetfd: int) -> None:
  L100: def start(self) -> None:
  L105: def stop(self) -> bytes:
  L117: class BytesIOCopy(io.BytesIO):
  L125: def __init__(self, copy_to: io.BytesIO) -> None:
  L129: def flush(self) -> None:
  L133: def write(self, b: ReadableBuffer) -> int:
  L138: class StreamMixer:
  L150: def __init__(self) -> None:
  L156: class _NamedTextIOWrapper(io.TextIOWrapper):
  L172: def __init__(
  L184: def close(self) -> None:
  L192: def fileno(self) -> int:
  L207: def mode(self) -> str:
  L211: def make_input_stream(
  L231: class Result:
  L262: def __init__(
  L283: def output(self) -> str:
  L295: def stdout(self) -> str:
  L302: def stderr(self) -> str:
  L312: def __repr__(self) -> str:
  L317: class CliRunner:
  L360: def __init__(
  L389: def make_env(
  L399: def isolation(
  L596: def invoke(
  L742: def isolated_filesystem(
src/click/core.py:
  L2462: def human_readable_name(self) -> str:
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L174: class ParameterSource(enum.IntEnum):
  L936: def set_parameter_source(self, name: str, source: ParameterSource) -> None:
  L945: def get_parameter_source(self, name: str) -> ParameterSource | None:
  L2216: class Parameter(ABC):
  L2335: def __init__(
  L2420: def _hide_unset(value: t.Any) -> t.Any:
  L2427: def to_info_dict(self) -> dict[str, t.Any]:
  L2453: def __repr__(self) -> str:
  L2457: def _parse_decls(
  L2468: def make_metavar(self, ctx: Context) -> str:
  L2483: def get_default(
  L2488: def get_default(
  L2492: def get_default(
  L2526: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None: ...
  L2528: def consume_value(
