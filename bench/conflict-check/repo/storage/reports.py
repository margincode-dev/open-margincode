SORTABLE = {"number", "issued_at", "total_cents"}


def invoices_sorted(conn, customer_id, sort):
    if sort not in SORTABLE:
        raise ValueError(f"cannot sort by {sort!r}")
    with conn.cursor() as cur:
        cur.execute(
            "SELECT number, issued_at, total_cents FROM invoices WHERE customer_id = %s "
            f"ORDER BY {sort}",
            (customer_id,),
        )
        return cur.fetchall()
