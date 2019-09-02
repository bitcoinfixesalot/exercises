pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut number = n;
    let mut candidate = 2;
    
    while number > 1 {
        while number % candidate == 0 {
            prime_factors.push(candidate);
            number /= candidate;
        }
        candidate += 1;
    }
    prime_factors
}
