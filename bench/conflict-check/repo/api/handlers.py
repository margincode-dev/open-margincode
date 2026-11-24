import json
import logging

from billing.invoices import invoice_total
from config import SUPPORTED_CURRENCIES
from storage import db

log = logging.getLogger("api")

JSON = {"Content-Type": "application/json"}


def error_response(status, code, message):
    return status, JSON, json.dumps({"error": code, "message": message})


def not_found(what):
    return 404, {"Content-Type": "text/plain"}, f"{what} not found"


def log_failed_request(request, reason):
    log.warning(
        "request failed: %s %s: %s body=%s",
        request.method,
        request.path,
        reason,
        request.body,
    )


def create_invoice_handler(request):
    payload = json.loads(request.body)
    customer = payload["customer"]
    log.info("creating invoice for %s <%s>", customer["name"], customer["email"])
    if payload.get("currency") not in SUPPORTED_CURRENCIES:
        log_failed_request(request, "unsupported currency")
        return error_response(400, "currency", "unsupported currency")
    conn = db.connect()
    found = db.find_customer(conn, customer["email"])
    if found is None:
        return error_response(422, "customer", "no such customer")
    invoice_id = db.insert_invoice(conn, found[0], payload["currency"])
    db.bulk_insert_lines(conn, invoice_id, payload["lines"])
    return 201, JSON, json.dumps({"id": invoice_id, "total": invoice_total(payload["lines"])})


def get_invoice_handler(request, invoice_id):
    invoice = db.get_invoice(db.connect(), invoice_id)
    if invoice is None:
        return not_found("invoice")
    return 200, JSON, json.dumps(invoice)
