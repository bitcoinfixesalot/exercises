use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input
        .chunks(input.len() / worker_count + 1)
        .collect::<Vec<&[&str]>>();

    input
        .par_iter()
        .map(|chunk| {
            let mut result: HashMap<char, usize> = HashMap::new();
            for slice in chunk.iter() {
                count_alphabetic(&mut result, slice)
            }
            result
        })
        .reduce(HashMap::new, |mut result, hmap| {
            hmap.iter().for_each(|(&key, &value)| {
                *result.entry(key).or_insert(0) += value;
            });
            result
        })
}

fn count_alphabetic(hmap: &mut HashMap<char, usize>, slice: &str) {
    for c in slice.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
        hmap.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
}
