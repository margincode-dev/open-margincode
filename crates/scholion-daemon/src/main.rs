use std::io::IsTerminal;

use axum::Router;
use scholion_daemon::{BoxError, Config, run_with};
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

async fn run() -> Result<(), BoxError> {
    run_with(Config::from_env()?, |state| async {
        Ok::<_, BoxError>((state, Router::new()))
    })
    .await
}
