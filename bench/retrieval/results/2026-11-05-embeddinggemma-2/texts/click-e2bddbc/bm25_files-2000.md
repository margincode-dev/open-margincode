### src/click/__init__.py
```python
"""
Click is a simple Python module inspired by the stdlib optparse to make
writing command line scripts fun. Unlike other modules, it's based
around a simple API that does not come with too much magic and is
composable.
"""

from __future__ import annotations

from .core import Argument as Argument
from .core import Command as Command
from .core import CommandCollection as CommandCollection
from .core import Context as Context
from .core import Group as Group
from .core import Option as Option
from .core import Parameter as Parameter
from .core import ParameterSource as ParameterSource
from .decorators import argument as argument
from .decorators import command as command
from .decorators import confirmation_option as confirmation_option
from .decorators import custom_version_option as custom_version_option
from .decorators import group as group
from .decorators import help_option as help_option
from .decorators import make_pass_decorator as make_pass_decorator
from .decorators import option as option
from .decorators import pass_context as pass_context
from .decorators import pass_obj as pass_obj
from .decorators import password_option as password_option
from .decorators import version_option as version_option
from .exceptions import Abort as Abort
from .exceptions import BadArgumentUsage as BadArgumentUsage
from .exceptions import BadOptionUsage as BadOptionUsage
from .exceptions import BadParameter as BadParameter
from .exceptions import ClickException as ClickException
from .exceptions import FileError as FileError
from .exceptions import MissingParameter as MissingParameter
from .exceptions import NoSuchCommand as NoSuchCommand
from .exceptions import NoSuchOption as NoSuchOption
from .exceptions import UsageError as UsageError
from .formatting import HelpFormatter as HelpFormatter
from .formatting import wrap_text as wrap_text
from .globals import get_current_context as get_current_context
from .termui import clear as clear
from .termui import confirm as confirm
from .termui import echo_via_pager as echo_via_pager
from .termui import edit as edit
from .termui import get_pager_file as get_pager_file
from .termui import getchar as getchar
from .termui import launch as launch
from .termui import pause as pause
from .termui import progressbar as progressbar
from .termui import prompt as prompt
from .termui import secho as secho
from .termui import style as style
from .termui import unstyle as unstyle
from .types import BOOL as BOOL
from .types import Choice as Choice
from .types import DateTime as DateTime
from .types import File as File
from .types import FLOAT as FLOAT
from .types import FloatRange as FloatRange
from .types import INT as INT
from .types import IntRange as IntRange
from .types import ParamType as ParamType
from .types import Path as Path
from .types import STRING as STRING
from .types import Tuple as Tuple
from .types import UNPROCESSED as UNPROCESSED
from .types import UUID as UUID
from .utils import echo as echo
from .utils import format_filename as format_filename
from .utils import get_app_dir as get_app_dir
from .utils import open_file as open_file


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
### src/click/parser.py
```python
"""
This module started out as largely a copy paste from the stdlib's
optparse module with the features removed that we do not need from
optparse because we implement them in Click on a higher level (for
instance type handling, help formatting and a lot more).

The plan is to remove more and more from here over time.

The reason this is a different module and not optparse from the stdlib
is that there are differences in 2.x and 3.x about the error messages
generated and optparse in the stdlib uses gettext for no good reason
and might cause us issues.

Click uses parts of optparse written by Gregory P. Ward and maintained
by the Python Software Foundation. This is limited to code in parser.py.

Copyright 2001-2006 Gregory P. Ward. All rights reserved.
Copyright 2002-2006 Python Software Foundation. All rights reserved.
"""

# This code uses parts of optparse written by Gregory P. Ward and
# maintained by the Python Software Foundation.
# Copyright 2001-2006 Gregory P. Ward
# Copyright 2002-2006 Python Software Foundation
from __future__ import annotations

import collections.abc as cabc
import typing as t
from collections import deque
from gettext import gettext as _
from gettext import ngettext

from ._utils import FLAG_NEEDS_VALUE
from ._utils import UNSET
from .exceptions import BadArgumentUsage
from .exceptions import BadOptionUsage
from .exceptions import NoSuchOption
from .exceptions import UsageError

if t.TYPE_CHECKING:
    from ._utils import T_FLAG_NEEDS_VALUE
    from ._utils import T_UNSET
    from .core import Argument as CoreArgument
    from .core import Context
    from .core import Option as CoreOption
    from .core import Parameter as CoreParameter

V = t.TypeVar("V")


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


def _split_opt(opt: str) -> tuple[str, str]:
    first = opt[:1]
