- Command.get_short_help_str (src/click/core.py:1247-1261)
```python
    def get_short_help_str(self, limit: int = 45) -> str:
        """Gets short help for the command or makes it by shortening the
        long help string.
        """
        if self.short_help:
            text = inspect.cleandoc(self.short_help)
        elif self.help:
            text = _make_default_short_help(self.help, limit)
        else:
            text = ""

        if self.deprecated:
            text = f"{_(text)} {_format_deprecated_label(self.deprecated)}"

        return text.strip()
```
- test_make_default_short_help (tests/test_utils/test_make_default_short_help.py:32-39)
```python
def test_make_default_short_help(value, max_length, alter, expect):
    assert len(expect) <= max_length

    if alter:
        value = alter(value)

    out = click.utils._make_default_short_help(value, max_length)
    assert out == expect
```
- test_auto_shorthelp (tests/test_commands.py:66-95)
```python
def test_auto_shorthelp(runner):
    @click.group()
    def cli():
        pass

    @cli.command()
    def short():
        """This is a short text."""

    @cli.command()
    def special_chars():
        """Login and store the token in ~/.netrc."""

    @cli.command()
    def long():
        """This is a long text that is too long to show as short help
        and will be truncated instead."""

    result = runner.invoke(cli, ["--help"])
    assert (
        re.search(
            r"Commands:\n\s+"
            r"long\s+This is a long text that is too long to show as short help"
            r"\.\.\.\n\s+"
            r"short\s+This is a short text\.\n\s+"
            r"special-chars\s+Login and store the token in ~/.netrc\.\s*",
            result.output,
        )
        is not None
    )
```
- Command.to_info_dict (src/click/core.py:1084-1093)
```python
    def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
        return {
            "name": self.name,
            "params": [param.to_info_dict() for param in self.get_params(ctx)],
            "help": self.help,
            "epilog": self.epilog,
            "short_help": self.short_help,
            "hidden": self.hidden,
            "deprecated": self.deprecated,
        }
```
- _make_default_short_help (src/click/utils.py:62-112)
```python
def _make_default_short_help(help: str, max_length: int = 45) -> str:
    """Returns a condensed version of help string.

    :meta private:
    """
    # Consider only the first paragraph.
    paragraph_end = help.find("\n\n")

    if paragraph_end != -1:
        help = help[:paragraph_end]

    # Collapse newlines, tabs, and spaces.
    words = help.split()

    if not words:
        return ""

    # The first paragraph started with a "no rewrap" marker, ignore it.
    if words[0] == "\b":
        words = words[1:]

    total_length = 0
    last_index = len(words) - 1

    for i, word in enumerate(words):
        total_length += len(word) + (i > 0)

        if total_length > max_length:  # too long, truncate
            break

        if word[-1] == ".":  # sentence end, truncate without "..."
            return " ".join(words[: i + 1])

        if total_length == max_length and i != last_index:
            break  # not at sentence end, truncate with "..."
    else:
        return " ".join(words)  # no truncation needed

    # Account for the length of the suffix.
    total_length += len("...")

    # remove words until the length is short enough
    while i > 0:
        total_length -= len(words[i]) + (i > 0)

        if total_length <= max_length:
            break

        i -= 1

    return " ".join(words[:i]) + "..."
```
- HELLO_COMMAND (tests/test_info_dict.py:67-78)
```python
HELLO_COMMAND = (
    click.Command("hello", params=[NUMBER_OPTION[0]]),
    {
        "name": "hello",
        "params": [NUMBER_OPTION[1], HELP_OPTION[1]],
        "help": None,
        "epilog": None,
        "short_help": None,
        "hidden": False,
        "deprecated": False,
    },
)
```
- HELLO_GROUP (tests/test_info_dict.py:79-92)
```python
HELLO_GROUP = (
    click.Group("cli", [HELLO_COMMAND[0]]),
    {
        "name": "cli",
        "params": [HELP_OPTION[1]],
        "help": None,
        "epilog": None,
        "short_help": None,
        "hidden": False,
        "deprecated": False,
        "commands": {"hello": HELLO_COMMAND[1]},
        "chain": False,
    },
)
```
- Command.__init__ (src/click/core.py:1040-1082)
```python
    def __init__(
        self,
        name: str | None,
        context_settings: cabc.MutableMapping[str, t.Any] | None = None,
        callback: t.Callable[..., t.Any] | None = None,
        params: list[Parameter] | None = None,
        help: str | None = None,
        epilog: str | None = None,
        short_help: str | None = None,
        options_metavar: str | None = "[OPTIONS]",
        add_help_option: bool = True,
        no_args_is_help: bool = False,
        hidden: bool = False,
        deprecated: bool | str = False,
    ) -> None:
        #: the name the command thinks it has.  Upon registering a command
        #: on a :class:`Group` the group will default the command name
        #: with this information.  You should instead use the
        #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
        self.name = name

        if context_settings is None:
            context_settings = {}

        #: an optional dictionary with defaults passed to the context.
        self.context_settings = context_settings

        #: the callback to execute when the command fires.  This might be
        #: `None` in which case nothing happens.
        self.callback = callback
        #: the list of parameters for this command in the order they
        #: should show up in the help page and execute.  Eager parameters
        #: will automatically be handled before non eager ones.
        self.params = params or []
        self.help = help
        self.epilog = epilog
        self.options_metavar = options_metavar
        self.short_help = short_help
        self.add_help_option = add_help_option
        self._help_option = None
        self.no_args_is_help = no_args_is_help
        self.hidden = hidden
        self.deprecated = deprecated
```
- test_removing_multiline_marker (tests/test_formatting.py:387-403)
```python
def test_removing_multiline_marker(runner):
    @click.group()
    def cli():
        pass

    @cli.command()
    def cmd1():
        """\b
        This is command with a multiline help text
        which should not be rewrapped.
        The output of the short help text should
        not contain the multiline marker.
        """
        pass

    result = runner.invoke(cli, ["--help"])
    assert "\b" not in result.output
```
- __getattr__ (src/click/utils.py:669-688)
```python
def __getattr__(name: str) -> object:
    import warnings

    if name in {
        "LazyFile",
        "KeepOpenFile",
        "make_default_short_help",
        "PacifyFlushWrapper",
        "safecall",
        "get_text_stream",
        "get_binary_stream",
    }:
        warnings.warn(
            f"'click.utils.{name}' is deprecated and will be removed in Click 9.0.",
            DeprecationWarning,
            stacklevel=2,
        )
        return globals()[f"_{name}"]

    raise AttributeError(name)
```
- Group.format_commands (src/click/core.py:1984-2010)
```python
    def format_commands(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Extra format methods for multi methods that adds all the commands
        after the options.
        """
        commands = []
        for subcommand in self.list_commands(ctx):
            cmd = self.get_command(ctx, subcommand)
            # What is this, the tool lied about a command.  Ignore it
            if cmd is None:
                continue
            if cmd.hidden:
                continue

            commands.append((subcommand, cmd))

        # allow for 3 times the default spacing
        if len(commands):
            limit = formatter.width - 6 - max(len(cmd[0]) for cmd in commands)

            rows = []
            for subcommand, cmd in commands:
