from datetime import datetime

from config import REMINDER_DAYS


def due_reminders(invoices):
    """The open invoices that get a reminder today."""
    today = datetime.now().date()
    return [inv for inv in invoices if (inv["due"].date() - today).days in REMINDER_DAYS]


def send_reminders(batch, mailer):
    for inv in batch:
        mailer.send(
            inv["customer_email"],
            f"Invoice {inv['number']} is due on {inv['due']:%d %B}",
        )
