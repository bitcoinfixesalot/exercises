#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: Vec<NucleotideDNA>,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: Vec<NucleotideRNA>,
}

#[derive(Debug, PartialEq)]
pub enum NucleotideRNA {
    A,
    C,
    G,
    U,
}

#[derive(Debug, PartialEq)]
pub enum NucleotideDNA {
    A,
    C,
    G,
    T,
}

impl NucleotideDNA {
    pub fn to_rna(&self) -> NucleotideRNA {
        match self {
            NucleotideDNA::A => NucleotideRNA::U,
            NucleotideDNA::C => NucleotideRNA::G,
            NucleotideDNA::G => NucleotideRNA::C,
            NucleotideDNA::T => NucleotideRNA::A,
        }
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut nucleotides = Vec::new();
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' => nucleotides.push(NucleotideDNA::A),
                'C' => nucleotides.push(NucleotideDNA::C),
                'G' => nucleotides.push(NucleotideDNA::G),
                'T' => nucleotides.push(NucleotideDNA::T),
                _ => return Err(i),
            }
        }
        Ok(DNA { nucleotides })
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            nucleotides: self.nucleotides.iter().map(|n| n.to_rna()).collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut nucleotides = Vec::new();
        for (i, c) in rna.chars().enumerate() {
            match c {
                'A' => nucleotides.push(NucleotideRNA::A),
                'C' => nucleotides.push(NucleotideRNA::C),
                'G' => nucleotides.push(NucleotideRNA::G),
                'U' => nucleotides.push(NucleotideRNA::U),
                _ => return Err(i),
            }
        }
        Ok(RNA { nucleotides })
    }
}
