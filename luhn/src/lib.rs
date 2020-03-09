/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let result = code
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
        });

    match result {
        Some((sum, count)) => count > 1 && sum % 10 == 0,
        None => false,
    }
    // let code = code
    //     .chars()
    //     .filter(|c| !c.is_whitespace())
    //     .collect::<Vec<char>>();

    //     if code.iter().all(|c| c.is_ascii_digit()) && code.len() > 1 {
    //     let sum = code
    //         .iter()
    //         .filter_map(|c| c.to_digit(10))
    //         .rev()
    //         .enumerate()
    //         .fold(0, |acc, (index, n)| {
    //             if index % 2 == 0 {
    //                 acc + n
    //             } else if n * 2 > 9 {
    //                 acc + n * 2 - 9
    //             } else {
    //                 acc + n * 2
    //             }
    //         });

    //     sum % 10 == 0
    // } else {
    //     false
    // }
}
