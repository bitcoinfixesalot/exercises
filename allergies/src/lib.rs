use self::Allergen::*;
use std::slice::Iter;

pub struct Allergies {
    score: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn iter() -> Iter<'static, Allergen> {
        static ALLERGENS: [Allergen; 8] = [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ];
        ALLERGENS.iter()
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = self.score;
        let allergen = *allergen as u32;
        score & allergen != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter().fold(Vec::new(), |mut vec, elem| {
            if self.is_allergic_to(&elem) {
                vec.push(*elem);
            }
            vec
        })
    }
}
