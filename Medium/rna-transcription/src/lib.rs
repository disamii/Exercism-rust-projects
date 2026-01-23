use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, letter) in dna.chars().enumerate() {
            if !"CGTA".contains(letter) {
                return Err(i);
            }
        }
        return Ok(Dna {
            sequence: dna.to_string(),
        });
    }

    pub fn into_rna(self) -> Rna {
        let sequence_rule = HashMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]);
        let mut rna = String::new();
        for c in self.sequence.chars() {
            if let Some(chars) = sequence_rule.get(&c) {
                rna.push(*chars);
            }
        }
        Rna { sequence: rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, letter) in rna.chars().enumerate() {
            if !"CGUA".contains(letter) {
                return Err(i);
            }
        }
        return Ok(Rna {
            sequence: rna.to_string(),
        });
    }
}
