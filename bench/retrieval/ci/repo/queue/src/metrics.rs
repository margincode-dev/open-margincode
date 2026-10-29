//! Counters and latency percentiles for the worker pool.
#[derive(Default)]
pub struct Counter {
    value: u64,
}

impl Counter {
    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn get(&self) -> u64 {
        self.value
    }
}

#[derive(Default)]
pub struct Latencies {
    samples: Vec<f64>,
}

impl Latencies {
    pub fn record_latency(&mut self, ms: f64) {
        self.samples.push(ms);
    }

    /// The `p`-th percentile (0-100) of the recorded samples, nearest rank.
    pub fn percentile(&self, p: f64) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let rank = (p / 100.0 * sorted.len() as f64) as usize;
        Some(sorted[rank.min(sorted.len() - 1)])
    }

    pub fn mean(&self) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        Some(self.samples.iter().sum::<f64>() / self.samples.len() as f64)
    }
}
