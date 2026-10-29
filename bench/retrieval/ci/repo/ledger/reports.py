"""Reports over a month of invoices."""
import csv


def monthly_summary(invoices, year, month):
    total = 0
    count = 0
    for inv in invoices:
        if inv.issued.year == year and inv.issued.month == month:
            total += inv.total
            count += 1
    return {"count": count, "total": total}


def aging_buckets(invoices, today):
    """Group unpaid invoices by how overdue they are: current, 30, 60 or 90 days."""
    buckets = {"current": [], "30": [], "60": [], "90": []}
    for inv in invoices:
        if inv.paid:
            continue
        overdue = (today - inv.due).days
        if overdue < 0:
            buckets["current"].append(inv)
        elif overdue < 30:
            buckets["30"].append(inv)
        elif overdue < 60:
            buckets["60"].append(inv)
        else:
            buckets["90"].append(inv)
    return buckets


def export_csv(invoices, path):
    with open(path, "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["number", "customer", "issued", "total"])
        for inv in invoices:
            w.writerow([inv.number, inv.customer, inv.issued.isoformat(), inv.total])
