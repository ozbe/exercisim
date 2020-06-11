use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    NUCLEOTIDES
        .iter()
        .find(|&&n| n == nucleotide)
        .ok_or(nucleotide)
        .and_then(|_| {
            nucleotide_counts(dna).and_then(|c| c.get(&nucleotide).cloned().ok_or(nucleotide))
        })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    NUCLEOTIDES.iter().for_each(|&n| {
        counts.insert(n, 0);
    });

    for n in dna.chars() {
        if let Some(v) = counts.get_mut(&n) {
            *v += 1;
        } else {
            return Err(n);
        }
    }

    Ok(counts)
}
