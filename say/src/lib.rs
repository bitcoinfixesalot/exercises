pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut exp = 0;

    let mut result = String::new();

    while n > 0 {
        let block = say_less_than_thousand(n % 1000);
        if block != "" {
            result.insert_str(0, &format!("{} {} ", block, thousand_or_more(exp)));
        }

        n /= 1000;
        exp += 3;
    }
    result.trim().to_string()
}

fn say_less_than_hundred(n: u64) -> String {
    if n < 20 {
        return one_to_nineteen(n).to_string();
    }

    if n % 10 == 0 {
        return tens(n).to_string();
    }

    format!("{}-{}", tens(n), one_to_nineteen(n % 10))
}

fn say_less_than_thousand(n: u64) -> String {
    if n == 0 {
        return String::new();
    }

    if n < 100 {
        return say_less_than_hundred(n);
    }

    format!(
        "{} hundred {}",
        one_to_nineteen(n / 100),
        say_less_than_hundred(n % 100)
    )
}

fn one_to_nineteen<'a>(n: u64) -> &'a str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }
}

fn tens<'a>(n: u64) -> &'a str {
    match (n % 100) / 10 {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    }
}

fn thousand_or_more<'a>(n: u64) -> &'a str {
    match n {
        3 => "thousand",
        6 => "million",
        9 => "billion",
        12 => "trillion",
        15 => "quadrillion",
        18 => "quintillion",
        _ => "",
    }
}