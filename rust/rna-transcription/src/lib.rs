use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let valid_nucleotides: HashSet<char> = create_dna_to_rna_map().keys().cloned().collect();

        if let Some(i) = first_invalid_nucleotide(dna, &valid_nucleotides) {
            Err(i)
        } else {
            Ok(DNA(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> RNA {
        let map = create_dna_to_rna_map();
        let rna: String = self
            .0
            .chars()
            .map(|c| {
                map.get(&c)
                    .unwrap_or_else(|| panic!("Unexpected nucleotide `{}`", c))
            })
            .collect();

        RNA(rna)
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let valid_nucleotides = create_dna_to_rna_map().values().cloned().collect();

        if let Some(i) = first_invalid_nucleotide(rna, &valid_nucleotides) {
            Err(i)
        } else {
            Ok(RNA(rna.to_string()))
        }
    }
}

fn create_dna_to_rna_map() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('G', 'C');
    map.insert('C', 'G');
    map.insert('T', 'A');
    map.insert('A', 'U');
    map
}

fn first_invalid_nucleotide(strand: &str, valid_nucleotides: &HashSet<char>) -> Option<usize> {
    strand.find(|c: char| !valid_nucleotides.contains(&c))
}
