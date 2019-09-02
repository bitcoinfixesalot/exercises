pub fn nth(n: u32) -> u32 {
    let mut numbers_found = vec![2];
    let mut current_number = 3;
    while numbers_found.len() <= (n as usize) {
        if !numbers_found.iter().any(|prime| current_number % prime == 0) {
            numbers_found.push(current_number);
        };
        current_number += 2;
    }
    numbers_found[n as usize]
}


// pub fn nth(n: u32) -> u32 {
//     let mut numbers_found = 0;
//     for number in 2..u32::max_value() {
//         if is_prime(number) {
//             numbers_found = numbers_found + 1;
//         }
//         if numbers_found == n + 1 {
//             return number;
//         }
//     }
//     0
// }

// fn is_prime(number: u32) -> bool {
//     for i in 2..number {
//         if number % i == 0 {
//             return false;
//         }
//     }
//     true
// }
