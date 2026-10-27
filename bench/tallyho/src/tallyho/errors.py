"""Exceptions raised by tallyho."""


class TallyError(Exception):
    """Base class of every tallyho error."""


class ValidationError(TallyError):
    """A value failed validation. ``field`` names the offending field, if known."""

    def __init__(self, message: str, field: str | None = None) -> None:
        super().__init__(message)
        self.field = field

    def __str__(self) -> str:
        base = super().__str__()
        return f"{self.field}: {base}" if self.field else base


class CurrencyMismatch(TallyError):
    """Two amounts in different currencies were combined."""


class NotFound(TallyError):
    """A record does not exist in the store."""


class ConfigError(TallyError):
    """The configuration file is malformed."""
