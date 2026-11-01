//! `serve_with` / `run_with`: the daemon's start-up as a library call, so a binary built on
//! Scholion starts like `scholion-daemon` and adds its own routes and migrations.

use std::time::Duration;

use axum::Router;
use axum::extract::State;
use axum::routing::get;
use scholion_daemon::{AppState, BoxError, Config, serve_with};
use scholion_testutil::TestDb;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

static EXT: sqlx::migrate::Migrator = sqlx::migrate!("./tests/ext_migrations");

fn config(db: &TestDb, dir: &std::path::Path, token: Option<&str>) -> Config {
    Config {
        database_url: Some(db.url.clone()),
        pg_dir: dir.join("pg"),
        pg_install_dir: None,
        store_root: dir.join("store"),
        ws_root: dir.join("ws"),
        ws_host_root: dir.join("ws"),
        listen: "127.0.0.1:0".into(),
        embed_api_key: None,
        embed: None,
        qdrant_url: None,
        expand_cap: None,
        pg_max_connections: 4,
        max_concurrent_starts: None,
        lease_secs: 1800,
        backup_remotes: Vec::new(),
        token: token.map(str::to_owned),
    }
}

async fn board_rows(State(st): State<AppState>) -> String {
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM ee.board")
        .fetch_one(st.pool())
        .await
        .unwrap();
    format!("rows={n}")
}

/// One HTTP/1.1 request on a raw socket: (status code, body).
async fn get_raw(addr: std::net::SocketAddr, path: &str, token: Option<&str>) -> (u16, String) {
    let mut s = tokio::net::TcpStream::connect(addr).await.unwrap();
    let auth = token
        .map(|t| format!("authorization: Bearer {t}\r\n"))
        .unwrap_or_default();
    let req = format!("GET {path} HTTP/1.1\r\nhost: x\r\n{auth}connection: close\r\n\r\n");
    s.write_all(req.as_bytes()).await.unwrap();
    let mut buf = String::new();
    s.read_to_string(&mut buf).await.unwrap();
    let code = buf[9..12].parse().unwrap();
    let body = buf.split("\r\n\r\n").nth(1).unwrap_or_default().to_owned();
    (code, body)
}

async fn wait_up(addr: std::net::SocketAddr) {
    for _ in 0..200 {
        if tokio::net::TcpStream::connect(addr).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("daemon never listened on {addr}");
}

#[tokio::test(flavor = "multi_thread")]
async fn serve_with_serves_core_and_extension_routes_and_runs_extension_migrations() {
    let db = TestDb::new().await;
    let dir = tempfile::tempdir().unwrap();
    let cfg = config(&db, dir.path(), Some("secret"));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();

    // Not spawned: the `customize` future holds sqlx borrows that are not `Send` for every
    // lifetime, which a binary's `main` never needs; the test drives server and client together.
    let server = serve_with(
        cfg,
        listener,
        |state: AppState| async move {
            state.migrate_extension("ee", &EXT).await?;
            let extra = Router::new().route("/v1/ee/board", get(board_rows));
            Ok::<_, BoxError>((state, extra))
        },
        async move {
            let _ = stopped.await;
        },
    );
    let client = async move {
        wait_up(addr).await;
        assert_eq!(get_raw(addr, "/healthz", None).await, (200, "ok".into()));
        // core route, behind the token
        assert_eq!(get_raw(addr, "/v1/repos", None).await.0, 401);
        let (code, body) = get_raw(addr, "/v1/repos", Some("secret")).await;
        assert_eq!(code, 200, "{body}");
        // the extension's route is behind the same token and reads its own schema
        assert_eq!(get_raw(addr, "/v1/ee/board", None).await.0, 401);
        assert_eq!(
            get_raw(addr, "/v1/ee/board", Some("secret")).await,
            (200, "rows=0".into())
        );
        stop.send(()).unwrap();
    };
    let (served, ()) = tokio::time::timeout(Duration::from_secs(60), async {
        tokio::join!(server, client)
    })
    .await
    .expect("daemon stops on the shutdown future");
    served.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn an_error_from_customize_stops_start_up_before_serving() {
    let db = TestDb::new().await;
    let dir = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let err = serve_with(
        config(&db, dir.path(), None),
        listener,
        |_state: AppState| async move {
            Err::<(AppState, Router<AppState>), BoxError>("no master key".into())
        },
        std::future::pending(),
    )
    .await
    .unwrap_err();
    assert_eq!(err.to_string(), "no master key");
}
