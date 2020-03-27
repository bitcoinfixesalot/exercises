use rayon::prelude::*;
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..(sum / 3))
        .into_par_iter()
        .filter_map(|a| {
            let b_c = sum - a;
            let b = (b_c.pow(2) - a.pow(2)) / (2 * b_c);
            let c = b_c - b;

            if a < b && a.pow(2) + b.pow(2) == c.pow(2) {
                Some([a, b, c])
            } else {
                None
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
