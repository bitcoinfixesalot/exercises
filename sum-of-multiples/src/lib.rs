pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|i| factors.into_iter().any(|f| *f != 0 && i % f == 0))
        .sum()
}
