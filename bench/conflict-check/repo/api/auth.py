import hmac
import logging
from datetime import datetime, timezone

log = logging.getLogger("auth")


def check_token(token, stored):
    """`stored` is the token's row, or None when no token of that value exists."""
    if stored is None:
        log.info("token rejected: unknown")
        return False
    if stored["revoked"]:
        log.info("token rejected: revoked customer=%s", stored["customer_id"])
        return False
    if stored["expires_at"] <= datetime.now(timezone.utc):
        log.info("token rejected: expired customer=%s", stored["customer_id"])
        return False
    return hmac.compare_digest(token, stored["token"])
