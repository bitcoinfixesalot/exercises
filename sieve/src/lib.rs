use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut not_prime: HashSet<u64> = HashSet::new();

    for number in 2..=upper_bound {
        if not_prime.contains(&number) {
            continue;
        }
        result.push(number);
        let mut j = number * 2;
        while j <= upper_bound {
            not_prime.insert(j);
            j += number;
        }
    }
    result
}

// pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
//     let mut result: Vec<u64> = (2..upper_bound + 1).collect();
//     for number in result.clone().into_iter() {
//         let mut j = number * 2;
//         while j <= upper_bound {
//             result.retain(|&x| x != j);
//             j += number;
//         }
//     }
//     result
// }
