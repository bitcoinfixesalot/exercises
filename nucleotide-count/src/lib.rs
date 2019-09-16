use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let counts = nucleotide_counts(dna)?;
    match counts.get(&nucleotide) {
        Some(n) => Ok(*n),
        None => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::from_iter(vec![('G', 0), ('T', 0), ('A', 0), ('C', 0)]);
    for c in dna.chars() {
        match result.entry(c) {
            Entry::Occupied(n) => *n.into_mut() += 1,
            Entry::Vacant(_) => return Err(c),
        }
    }
    Ok(result)
}
