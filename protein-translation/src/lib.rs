use std::collections::BTreeMap;
use std::iter::FromIterator;

pub struct CodonsInfo<'a> {
    bmap: BTreeMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.bmap.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut buffer = rna;
        let mut names: Vec<&'a str> = Vec::new();
        while let Some(name) = self.name_for(&buffer[..3]) {
            if name == "stop codon" {
                break;
            }
            buffer = &buffer[3..];
            names.push(name);
            if buffer.len() < 3 {
                break;
            }
        }
        if names.is_empty() {
            None
        } else {
            Some(names)
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        bmap: BTreeMap::from_iter(pairs.into_iter()),
    }
}
