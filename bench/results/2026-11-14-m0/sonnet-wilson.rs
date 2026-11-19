// Prints margincode_bench::stats::wilson_interval for each ACCEPTED/TRIALS argument, compiled
// against that very file of the run tree (7dd5208):
//   rustc -O --edition 2024 sonnet-wilson.rs -o wilson && ./wilson 11/30 30/30
#[allow(dead_code)]
#[path = "/m2b/m0s/tree/crates/margincode-bench/src/stats.rs"]
mod stats;

fn main() {
    for arg in std::env::args().skip(1) {
        let (k, n) = arg.split_once('/').expect("ACCEPTED/TRIALS");
        let (k, n): (u64, u64) = (k.parse().unwrap(), n.parse().unwrap());
        match stats::wilson_interval(k, n) {
            Some((lo, hi)) => println!("{k}/{n}\t{:.3}\t{lo:.3} to {hi:.3}", k as f64 / n as f64),
            None => println!("{k}/{n}\tno interval"),
        }
    }
}
