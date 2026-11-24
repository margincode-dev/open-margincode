import os

DATABASE_URL = os.environ.get("DATABASE_URL", "postgresql://localhost/tidepool")

# rows per page in list responses
PAGE_SIZE = 50

TOKEN_TTL_HOURS = 12

# days before the due date on which a reminder goes out
REMINDER_DAYS = (7, 1)

SUPPORTED_CURRENCIES = {"EUR", "USD", "GBP"}
