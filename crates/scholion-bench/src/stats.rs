//! Medians and bootstrap confidence intervals, with a small deterministic PRNG (so reports and the
//! arm order of a run are reproducible from `--seed`).

/// xorshift64*: deterministic, good enough for shuffles and resampling.
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in `0..n` (`n > 0`).
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            v.swap(i, self.below(i + 1));
        }
    }
}

pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut v = values.to_vec();
    v.sort_by(f64::total_cmp);
    let n = v.len();
    Some(if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    })
}

/// Percentile bootstrap 95% CI of the median (2000 resamples). `None` with fewer than 2 values.
pub fn bootstrap_median_ci(values: &[f64], seed: u64) -> Option<(f64, f64)> {
    if values.len() < 2 {
        return None;
    }
    let mut rng = Rng::new(seed);
    let mut meds: Vec<f64> = (0..2000)
        .map(|_| {
            let sample: Vec<f64> = (0..values.len())
                .map(|_| values[rng.below(values.len())])
                .collect();
            median(&sample).expect("non-empty")
        })
        .collect();
    meds.sort_by(f64::total_cmp);
    Some((meds[49], meds[1949]))
}

/// `(new - base) / base`, `None` when `base` is missing or 0.
pub fn relative_change(base: Option<f64>, new: Option<f64>) -> Option<f64> {
    match (base, new) {
        (Some(b), Some(n)) if b != 0.0 => Some((n - b) / b),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_of_odd_even_and_empty() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]), Some(2.5));
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn bootstrap_ci_brackets_the_median_and_is_reproducible() {
        let v: Vec<f64> = (1..=21).map(f64::from).collect();
        let (lo, hi) = bootstrap_median_ci(&v, 7).unwrap();
        assert!(lo <= 11.0 && 11.0 <= hi, "{lo} {hi}");
        assert_eq!(bootstrap_median_ci(&v, 7), Some((lo, hi)));
        assert_eq!(bootstrap_median_ci(&[5.0], 7), None);
    }

    #[test]
    fn relative_change_never_divides_by_zero() {
        assert_eq!(relative_change(Some(200.0), Some(150.0)), Some(-0.25));
        assert_eq!(relative_change(Some(0.0), Some(1.0)), None);
        assert_eq!(relative_change(None, Some(1.0)), None);
    }

    #[test]
    fn shuffle_is_a_seeded_permutation() {
        let mut a = [0, 1, 2, 3, 4, 5];
        Rng::new(3).shuffle(&mut a);
        let mut b = [0, 1, 2, 3, 4, 5];
        Rng::new(3).shuffle(&mut b);
        assert_eq!(a, b);
        let mut sorted = a;
        sorted.sort();
        assert_eq!(sorted, [0, 1, 2, 3, 4, 5]);
    }
}
