use std::collections::HashMap;
use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    (0..worker_count)
    .into_par_iter()
    .map(|i| {
        let mut letters = HashMap::new();
        input.join("")
            .chars()
            .skip(i)
            .step_by(worker_count)
            .filter(|c| c.is_alphabetic())
            .flat_map(|c| c.to_lowercase())
            .for_each(|c| {
                *letters.entry(c).or_insert(0) += 1;
            });
        letters
    })
    .reduce(HashMap::new, |mut result, m| {
        m.iter().for_each(|(&k, &v)| {
            *result.entry(k).or_insert(0) += v;
        });
        result
    })
}
