pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => {
            let mut n = n;
            let mut steps = 0;
            while n != 1 {
                if n % 2 == 0 {
                    n /= 2;
                } else {
                    n = n * 3 + 1;
                }
                steps += 1;
            }
            Some(steps)
        }
    }
}
