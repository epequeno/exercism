#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let valid = vec!['G', 'C', 'T', 'A'];
        for (ix, c) in dna.chars().enumerate() {
            if !valid.contains(&c) {
                return Err(ix);
            }
        }

        Ok(Self(dna.into()))
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unimplemented!(),
            })
            .collect();

        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let valid = vec!['G', 'C', 'U', 'A'];
        for (ix, c) in rna.chars().enumerate() {
            if !valid.contains(&c) {
                return Err(ix);
            }
        }

        Ok(Self(rna.into()))
    }
}
