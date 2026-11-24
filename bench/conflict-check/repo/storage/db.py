import psycopg2

from config import DATABASE_URL


def connect():
    return psycopg2.connect(DATABASE_URL)


def find_customer(conn, email):
    with conn.cursor() as cur:
        cur.execute(
            f"SELECT id, name, email FROM customers WHERE lower(email) = lower('{email}')"
        )
        return cur.fetchone()


def insert_invoice(conn, customer_id, currency):
    with conn.cursor() as cur:
        cur.execute(
            "INSERT INTO invoices (customer_id, currency, state) VALUES (%s, %s, 'draft') "
            "RETURNING id",
            (customer_id, currency),
        )
        return cur.fetchone()[0]


def get_invoice(conn, invoice_id):
    with conn.cursor() as cur:
        cur.execute(
            "SELECT id, number, customer_id, total_cents, state FROM invoices WHERE id = %s",
            (invoice_id,),
        )
        return cur.fetchone()


def count_open_invoices(conn, customer_id):
    with conn.cursor() as cur:
        cur.execute(
            "SELECT count(*) FROM invoices WHERE customer_id = %s AND state = %s",
            (customer_id, "open"),
        )
        return cur.fetchone()[0]


def invoices_issued_on(conn, day):
    with conn.cursor() as cur:
        cur.execute(
            "SELECT number, customer_id, total_cents, state FROM invoices "
            "WHERE issued_at::date = %s",
            (day,),
        )
        cols = ["number", "customer_id", "total_cents", "state"]
        return [dict(zip(cols, row)) for row in cur.fetchall()]


def bulk_insert_lines(conn, invoice_id, lines):
    values = ", ".join(
        f"({invoice_id}, '{line['description']}', {line['price']}, {line['qty']})"
        for line in lines
    )
    with conn.cursor() as cur:
        cur.execute(
            f"INSERT INTO invoice_lines (invoice_id, description, price, qty) VALUES {values}"
        )
