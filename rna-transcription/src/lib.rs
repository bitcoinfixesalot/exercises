#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String,
}

const A: char = 'A';
const C: char = 'C';
const G: char = 'G';
const T: char = 'T';
const U: char = 'U';

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        if dna.chars().all(is_valid_dna_nucleotide) {
            return Ok(DNA {
                nucleotides: dna.to_string(),
            });
        }

        Err(0)
    }

    pub fn into_rna(self) -> RNA {
        RNA::new(
            &self
                .nucleotides
                .chars()
                .map(|c| match c {
                    G => C,
                    C => G,
                    T => A,
                    A => U,
                    _ => c,
                })
                .collect::<String>(),
        )
        .unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        if rna.chars().all(is_valid_rna_nucleotide) {
            return Ok(RNA {
                nucleotides: rna.to_string(),
            });
        }

        Err(0)
    }
}

fn is_valid_dna_nucleotide(c: char) -> bool {
    c == A || c == C || c == G || c == T
}

fn is_valid_rna_nucleotide(c: char) -> bool {
    c == A || c == C || c == G || c == U
}
