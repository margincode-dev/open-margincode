#!/usr/bin/env python3
# fake.py PORT MODE: hang | 500 | 401 | slow:SECS (then 200 {}) on every request
import socket, sys, threading, time

port, mode = int(sys.argv[1]), sys.argv[2]


def serve(conn):
    try:
        conn.settimeout(5)
        try:
            conn.recv(65536)
        except Exception:
            pass
        if mode == "hang":
            time.sleep(3600)
            return
        if mode.startswith("slow:"):
            time.sleep(float(mode.split(":")[1]))
            status, body = "200 OK", b"{}"
        elif mode == "500":
            status, body = "500 Internal Server Error", b'{"error":"boom"}'
        else:
            status, body = "401 Unauthorized", b'{"error":"missing or wrong token"}'
        conn.sendall(
            f"HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {len(body)}\r\nconnection: close\r\n\r\n".encode()
            + body
        )
    finally:
        conn.close()


s = socket.socket()
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(("127.0.0.1", port))
s.listen(128)
while True:
    c, _ = s.accept()
    threading.Thread(target=serve, args=(c,), daemon=True).start()
