use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut amount = 0;
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => {}
        invalid => return Err(invalid),
    }

    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {
                if c == nucleotide {
                    amount += 1;
                }
            }
            invalid => return Err(invalid),
        }
    }
    Ok(amount)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_count = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);

    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {
                *nucleotide_count.get_mut(&c).unwrap() += 1;
            }
            invalid => return Err(invalid),
        }
    }

    Ok(nucleotide_count)
}
