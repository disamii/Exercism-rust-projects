use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    palindrome: u64,
    products: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(palindrome: u64, products: HashSet<(u64, u64)>) -> Self {
        Palindrome {
            palindrome,
            products,
        }
    }
    pub fn value(&self) -> u64 {
        self.palindrome
        // todo!("return the value of the palindrome")
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.products.clone()
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let mut palindromes: BTreeMap<u64, HashSet<(u64, u64)>> = BTreeMap::new();

    for a in min..=max {
        for b in a..=max {
            let product = a * b;
            if is_palindrome(product) {
                palindromes
                    .entry(product)
                    .or_insert_with(HashSet::new)
                    .insert((a, b));
            }
        }
    }

    let (min_value, min_factors) = palindromes.iter().next()?;
    let (max_value, max_factors) = palindromes.iter().next_back()?;

    Some((
        Palindrome::new(*min_value, min_factors.clone()),
        Palindrome::new(*max_value, max_factors.clone()),
    ))
}
fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
