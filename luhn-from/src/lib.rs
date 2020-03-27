pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.code
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .enumerate()
            .try_fold((0, 0), |(sum, count), (index, c)| {
                c.to_digit(10).map(|number| {
                    let mut acc = if index % 2 == 1 { number * 2 } else { number };
                    if acc > 9 {
                        acc -= 9;
                    }
                    (sum + acc, count + 1)
                })
            })
            .map_or(false, |(sum, count)| count > 1 && sum % 10 == 0)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
