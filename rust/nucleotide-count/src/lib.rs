use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid: Vec<char> = vec!['A', 'T', 'G', 'C'];
    if !valid.contains(&nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !valid.contains(&c) {
            return Err(c);
        }
    }

    let res = dna
        .chars()
        .map(|c| if nucleotide == c { 1 } else { 0 })
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    Ok(res)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res: HashMap<char, usize> = HashMap::new();
    let valid: Vec<char> = vec!['A', 'T', 'G', 'C'];
    for c in dna.chars() {
        if !valid.contains(&c) {
            return Err(c);
        }
    }

    valid.into_iter().for_each(|c| {
        let count = count(c, dna).unwrap_or_default();
        res.insert(c, count);
    });

    Ok(res)
}
