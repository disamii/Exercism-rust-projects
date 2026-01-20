pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    const ALLERGEN_VALUES: [(Allergen, u32); 8] = [
        (Allergen::Eggs, 1),
        (Allergen::Peanuts, 2),
        (Allergen::Shellfish, 4),
        (Allergen::Strawberries, 8),
        (Allergen::Tomatoes, 16),
        (Allergen::Chocolate, 32),
        (Allergen::Pollen, 64),
        (Allergen::Cats, 128),
    ];

    pub fn new(score: u32) -> Self {
        Allergies { score: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
             let value = Self::ALLERGEN_VALUES
            .iter()
            .find(|(a, _)| a == allergen)
            .map(|(_, v)| *v)
            .unwrap_or(0);

        (self.score & value) != 0
    }

pub fn allergies(&self) -> Vec<Allergen> {
        Self::ALLERGEN_VALUES
            .iter()
            .filter(|(_, value)| (self.score & value) != 0)
            .map(|(allergen, _)| *allergen) // Copy the allergen
            .collect()
    }
}
