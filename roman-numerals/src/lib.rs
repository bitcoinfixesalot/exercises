use std::fmt::{Display, Formatter, Result};

const NUMERALS: &[(&str, u32)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

pub struct Roman {
    numeral: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.numeral)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let numeral = NUMERALS
            .iter()
            .map(|(n, val)| {
                let count = num / val;
                num -= count * val;

                n.repeat(count as usize)
            })
            .collect::<String>();

        Roman { numeral }
    }
}
