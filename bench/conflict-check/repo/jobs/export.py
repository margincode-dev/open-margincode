import csv
import os
import tempfile
from datetime import datetime

from storage import db


def export_invoices(conn, day, out_dir="exports"):
    rows = [inv for inv in db.invoices_issued_on(conn, day) if inv["state"] != "draft"]
    exported_at = datetime.now().isoformat(timespec="seconds")
    fd, tmp = tempfile.mkstemp(dir=out_dir, suffix=".csv")
    with os.fdopen(fd, "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["number", "customer_id", "total_cents", "exported_at"])
        for inv in rows:
            w.writerow([inv["number"], inv["customer_id"], inv["total_cents"], exported_at])
    os.replace(tmp, os.path.join(out_dir, f"{day.isoformat()}.csv"))
