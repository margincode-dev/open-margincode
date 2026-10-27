use scholion_bench::stats::median;

#[test]
fn even_counts_average_the_two_middle_values() {
    assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]), Some(2.5));
    assert_eq!(median(&[10.0, 20.0]), Some(15.0));
}

#[test]
fn odd_and_empty() {
    assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
    assert_eq!(median(&[]), None);
}
