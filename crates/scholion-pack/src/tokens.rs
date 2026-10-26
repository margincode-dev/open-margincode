use std::sync::OnceLock;

use scholion_core::AgentFamily;
use tiktoken_rs::cl100k_base_singleton;

/// Per-family multiplier applied to the cl100k count. Claude's tokenizer is not public, so these are
/// calibration knobs (Plan 4 tunes them): `AV_TOKEN_FACTOR_CLAUDE` / `AV_TOKEN_FACTOR_OTHER`, default 1.0.
fn factor(family: AgentFamily) -> f64 {
    static FACTORS: OnceLock<(f64, f64)> = OnceLock::new();
    let read = |var: &str| {
        std::env::var(var)
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|f| *f > 0.0)
            .unwrap_or(1.0)
    };
    let (claude, other) = FACTORS.get_or_init(|| {
        (
            read("AV_TOKEN_FACTOR_CLAUDE"),
            read("AV_TOKEN_FACTOR_OTHER"),
        )
    });
    match family {
        AgentFamily::Claude => *claude,
        AgentFamily::Other => *other,
    }
}

/// Estimated token count of `text` for an agent of `family` (no safety margin; see `allocate`).
pub fn estimate_tokens(text: &str, family: AgentFamily) -> u32 {
    if text.is_empty() {
        return 0;
    }
    let raw = cl100k_base_singleton().encode_ordinary(text).len() as f64;
    (raw * factor(family)).ceil() as u32
}
