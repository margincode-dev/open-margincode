//! Token-factor calibration (Plan 4 Task 6): how many real Claude input tokens a pack costs per
//! cl100k token the packer estimates. The recommended value goes into `AV_TOKEN_FACTOR_CLAUDE`
//! (read once at daemon start).

use crate::Result;
use crate::stats::median;

/// One pack: its cl100k count and the input tokens a probe call with it measured (overhead
/// included).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Probe {
    pub cl100k: u64,
    pub measured_with_overhead: u64,
}

/// `(measured - overhead) / cl100k` per pack, skipping packs that measure nothing above the
/// overhead or have no tokens (their ratio is undefined).
pub fn ratios(probes: &[Probe], overhead: u64) -> Vec<f64> {
    probes
        .iter()
        .filter_map(|p| {
            let measured = p.measured_with_overhead.saturating_sub(overhead);
            (p.cl100k > 0 && measured > 0).then(|| measured as f64 / p.cl100k as f64)
        })
        .collect()
}

/// The median ratio, rounded up to two decimals.
pub fn recommended_factor(probes: &[Probe], overhead: u64) -> Result<f64> {
    let r = ratios(probes, overhead);
    let m = median(&r)
        .ok_or("no pack measured above the empty-prompt overhead; nothing to calibrate")?;
    Ok((m * 100.0 - 1e-9).ceil() / 100.0)
}

/// `(tiktoken estimate - measured) / measured`, `None` when nothing was measured.
pub fn estimate_error(cl100k: u64, measured: u64) -> Option<f64> {
    (measured > 0).then(|| (cl100k as f64 - measured as f64) / measured as f64)
}

pub fn cl100k_tokens(text: &str) -> u64 {
    tiktoken_rs::cl100k_base_singleton()
        .encode_with_special_tokens(text)
        .len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(cl100k: u64, measured: u64) -> Probe {
        Probe {
            cl100k,
            measured_with_overhead: measured,
        }
    }

    #[test]
    fn ratio_and_median_computed_from_pairs() {
        let probes = [p(100, 1110), p(200, 1240), p(400, 1460)];
        assert_eq!(ratios(&probes, 1000), vec![1.1, 1.2, 1.15]);
        assert_eq!(recommended_factor(&probes, 1000).unwrap(), 1.15);
    }

    #[test]
    fn recommended_factor_rounds_up_to_two_decimals() {
        let probes = [p(300, 1000 + 334)];
        assert_eq!(recommended_factor(&probes, 1000).unwrap(), 1.12);
        assert_eq!(recommended_factor(&[p(100, 1110)], 1000).unwrap(), 1.1);
    }

    #[test]
    fn overhead_is_subtracted_once() {
        let probes = [p(100, 5100), p(100, 5100)];
        assert_eq!(ratios(&probes, 5000), vec![1.0, 1.0]);
    }

    #[test]
    fn empty_input_set_is_an_error_not_a_nan() {
        assert!(recommended_factor(&[], 1000).is_err());
        assert!(recommended_factor(&[p(0, 2000), p(50, 900)], 1000).is_err());
        assert_eq!(estimate_error(10, 0), None);
        assert_eq!(estimate_error(90, 100), Some(-0.1));
    }

    #[test]
    fn cl100k_counts_tokens() {
        assert!(cl100k_tokens("hello world") >= 2);
    }
}
