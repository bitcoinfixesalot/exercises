use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        let value = a * b;
        let mut factors = Vec::new();
        factors.push((a, b));
        Palindrome { value, factors }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: Option<(Palindrome, Palindrome)> = None;
    for a in min..=max {
        for b in a..=max {
            let number = a * b;
            if is_palindrome(number) {
                if let Some((min_palindrome, max_palindrome)) = &mut palindromes {
                    match number.cmp(&min_palindrome.value()) {
                        Ordering::Less => *min_palindrome = Palindrome::new(a, b),
                        Ordering::Equal => min_palindrome.insert(a, b),
                        Ordering::Greater => {}
                    }
                    match number.cmp(&max_palindrome.value()) {
                        Ordering::Greater => *max_palindrome = Palindrome::new(a, b),
                        Ordering::Equal => max_palindrome.insert(a, b),
                        Ordering::Less => {}
                    }
                } else {
                    palindromes = Some((Palindrome::new(a, b), Palindrome::new(a, b)))
                }
            }
        }
    }
    palindromes
}

fn reverse_number(number: u64) -> u64 {
    let mut reverse = 0;
    let mut n = number;
    while n > 0 {
        reverse = 10 * reverse + n % 10;
        n /= 10;
    }
    reverse
}

fn is_palindrome(number: u64) -> bool {
    number == reverse_number(number)
}
