def search_customers(conn, term, limit=20):
    with conn.cursor() as cur:
        cur.execute(
            f"SELECT id, name FROM customers WHERE name ILIKE '%%{term}%%' "
            "ORDER BY name LIMIT %s",
            (limit,),
        )
        return cur.fetchall()
