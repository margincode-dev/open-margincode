//! Queue settings read from `key=value` lines.
use std::time::Duration;

pub struct Config {
    pub lease: Duration,
    pub workers: usize,
}

pub fn default_timeout() -> Duration {
    Duration::from_secs(30)
}

pub fn parse_config(text: &str) -> Config {
    let mut cfg = Config {
        lease: default_timeout(),
        workers: 4,
    };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key.trim() {
            "lease_secs" => {
                if let Ok(n) = value.trim().parse() {
                    cfg.lease = Duration::from_secs(n);
                }
            }
            "workers" => {
                if let Ok(n) = value.trim().parse() {
                    cfg.workers = n;
                }
            }
            _ => {}
        }
    }
    cfg
}
