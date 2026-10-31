use std::io::IsTerminal;
use std::time::Duration;

use scholion_daemon::{
    BACKUP_TICK, Config, EmbedChoice, EmbeddedPg, backup_tick, build_state, router, start_backups,
    sync_repos, worker_tick,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // `av up` sends the log to a file
        .with_ansi(std::io::stdout().is_terminal())
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,jj_lib=warn")),
        )
        .init();
    if let Err(e) = run().await {
        eprintln!("scholion-daemon: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::from_env()?;
    std::fs::create_dir_all(&cfg.store_root)?;
    std::fs::create_dir_all(&cfg.ws_root)?;
    // Bound before lite mode touches its Postgres: a second daemon fails here instead of stopping
    // the server the first one runs.
    let listener = tokio::net::TcpListener::bind(&cfg.listen).await?;
    let (database_url, embedded) = match &cfg.database_url {
        Some(url) => (url.clone(), None),
        None => {
            let pg = EmbeddedPg::start(&cfg.pg_dir, cfg.pg_install_dir.clone()).await?;
            tracing::info!(dir = %cfg.pg_dir.display(), "lite mode: embedded postgres started");
            (pg.url().to_owned(), Some(pg))
        }
    };
    let state = build_state(&cfg, &database_url).await?;
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
    tracing::info!(listen = %cfg.listen, ws_host_root = %cfg.ws_host_root.display(), "scholion-daemon ready");
    let served = axum::serve(listener, router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await;
    if let Some(task) = drain {
        task.abort();
    }
    backups.abort();
    if let Some(pg) = embedded {
        pg.stop().await?;
    }
    served?;
    Ok(())
}

/// Ctrl-C, or SIGTERM (`av down`, `docker stop`).
async fn shutdown_signal() {
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
