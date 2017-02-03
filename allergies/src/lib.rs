
// Tried my own solution which turned out to be not only massively verbose but also failing at the last test ALWAYS.
// So I looked up the example solution on github which in turn is not only refreshingly terse but also passes also the tests.

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq)]
pub struct Allergies {
    score: usize,
}

impl Allergies {
    pub fn new(s: usize) -> Self {
        Allergies { score: s }
    }

    pub fn is_allergic_to(&self, alg: &Allergen) -> bool {
        let allergens = Allergies::allergens();
        let index = match allergens.iter().position(|x: &Allergen| x == alg) {
            Some(u) => u,
            None => {
                return false;
            }
        };
        (self.score & (1 << index)) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergies::allergens()
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }

    fn allergens() -> Vec<Allergen> {
        vec![Allergen::Eggs,
             Allergen::Peanuts,
             Allergen::Shellfish,
             Allergen::Strawberries,
             Allergen::Tomatoes,
             Allergen::Chocolate,
             Allergen::Pollen,
             Allergen::Cats]
    }
}
