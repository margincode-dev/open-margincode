//! The daemon's start-up as a library call. `scholion-daemon` is `run_with` with no extras; a
//! binary built on Scholion (the commercial edition's daemon) calls it with a `customize` step that
//! adds its services, migrations and routes, so it starts exactly like the Community daemon.

use std::future::Future;
use std::time::Duration;

use axum::Router;
use tokio::net::TcpListener;

use crate::{
    AppState, BACKUP_TICK, Config, EmbedChoice, EmbeddedPg, backup_tick, build_state, router_with,
    start_backups, sync_repos, worker_tick,
};

/// Errors from start-up and from an extension's `customize` step.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Binds `cfg.listen`, then [`serve_with`] until Ctrl-C or SIGTERM (`av down`, `docker stop`).
///
/// The listener is bound before lite mode touches its Postgres: a second daemon fails here instead
/// of stopping the server the first one runs.
pub async fn run_with<F, Fut>(cfg: Config, customize: F) -> Result<(), BoxError>
where
    F: FnOnce(AppState) -> Fut,
    Fut: Future<Output = Result<(AppState, Router<AppState>), BoxError>>,
{
    std::fs::create_dir_all(&cfg.store_root)?;
    std::fs::create_dir_all(&cfg.ws_root)?;
    let listener = TcpListener::bind(&cfg.listen).await?;
    serve_with(cfg, listener, customize, shutdown_signal()).await
}

/// Starts the daemon on `listener` and serves until `shutdown` completes.
///
/// Order: Postgres (lite mode starts its own when `cfg.database_url` is unset), the core state,
/// the `repos` table sync, then `customize`, which gets the built state and returns it (with any
/// `with_extension`, `with_merge_guard`, `with_usage_sink`, `with_mcp_tools` added, and after any
/// `migrate_extension`) plus extra routes. Those are served through [`router_with`], behind the
/// same token as `/v1`. An error from `customize` stops start-up before anything is served. Then
/// the outbox worker and backups start, and the server runs.
///
/// The returned future is not `Send` when `customize` awaits sqlx queries (a higher-ranked
/// lifetime limit of sqlx), so await it in `main` rather than `tokio::spawn` it.
pub async fn serve_with<F, Fut, S>(
    cfg: Config,
    listener: TcpListener,
    customize: F,
    shutdown: S,
) -> Result<(), BoxError>
where
    F: FnOnce(AppState) -> Fut,
    Fut: Future<Output = Result<(AppState, Router<AppState>), BoxError>>,
    S: Future<Output = ()> + Send + 'static,
{
    std::fs::create_dir_all(&cfg.store_root)?;
    std::fs::create_dir_all(&cfg.ws_root)?;
    let (database_url, embedded) = match &cfg.database_url {
        Some(url) => (url.clone(), None),
        None => {
            let pg = EmbeddedPg::start(&cfg.pg_dir, cfg.pg_install_dir.clone()).await?;
            tracing::info!(dir = %cfg.pg_dir.display(), "lite mode: embedded postgres started");
            (pg.url().to_owned(), Some(pg))
        }
    };
    let served = serve_state(&cfg, &database_url, listener, customize, shutdown).await;
    if let Some(pg) = embedded {
        pg.stop().await?;
    }
    served
}

/// Everything between Postgres being up and the server stopping; lite mode's Postgres is stopped
/// by the caller whether this succeeds or not.
async fn serve_state<F, Fut, S>(
    cfg: &Config,
    database_url: &str,
    listener: TcpListener,
    customize: F,
    shutdown: S,
) -> Result<(), BoxError>
where
    F: FnOnce(AppState) -> Fut,
    Fut: Future<Output = Result<(AppState, Router<AppState>), BoxError>>,
    S: Future<Output = ()> + Send + 'static,
{
    let state = build_state(cfg, database_url).await?;
    // repos imported before the `repos` table existed: list them too (never fatal)
    match sync_repos(&state).await {
        Ok(0) => {}
        Ok(n) => tracing::info!(added = n, "repos table: added repos found on disk"),
        Err(e) => tracing::warn!(error = %e, "repos table: could not sync with the store on disk"),
    }
    match &cfg.embed {
        Some(EmbedChoice::Ollama { model, base_url }) => {
            tracing::info!(%model, %base_url, "embedder: ollama");
        }
        Some(EmbedChoice::Api { base_url, api_key }) => {
            tracing::info!(%base_url, api_key = api_key.is_some(), "embedder: api");
        }
        None => tracing::info!("embedder: off"),
    }
    let (state, extra) = customize(state).await?;
    let drain = match state.worker() {
        Some(worker) => {
            tracing::info!(qdrant = ?cfg.qdrant_url, "semantic layer on: outbox worker started");
            let pool = state.pool().clone();
            Some(tokio::spawn(async move {
                loop {
                    if let Err(e) = worker_tick(&pool, &worker).await {
                        tracing::warn!(error = %e, "outbox drain failed");
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }))
        }
        None => {
            tracing::info!("semantic layer off (needs an embedder and AV_QDRANT_URL)");
            None
        }
    };
    start_backups(&state, &cfg.backup_remotes).await?;
    let backups = {
        let state = state.clone();
        tokio::spawn(async move {
            let mut every = tokio::time::interval(BACKUP_TICK);
            loop {
                every.tick().await;
                if let Err(e) = backup_tick(&state).await {
                    tracing::warn!(error = %e, "backup retry failed");
                }
            }
        })
    };
    tracing::info!(
        listen = %listener.local_addr().map(|a| a.to_string()).unwrap_or_else(|_| cfg.listen.clone()),
        ws_host_root = %cfg.ws_host_root.display(),
        "scholion-daemon ready"
    );
    let served = axum::serve(listener, router_with(state, extra))
        .with_graceful_shutdown(shutdown)
        .await;
    if let Some(task) = drain {
        task.abort();
    }
    backups.abort();
    served?;
    Ok(())
}

/// Ctrl-C, or SIGTERM (`av down`, `docker stop`).
pub async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut term =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).ok();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = async {
                match term.as_mut() {
                    Some(t) => { t.recv().await; }
                    None => std::future::pending::<()>().await,
                }
            } => {}
        }
    }
    #[cfg(not(unix))]
    let _ = tokio::signal::ctrl_c().await;
}
