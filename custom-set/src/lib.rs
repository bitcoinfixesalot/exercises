use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Eq + Hash> {
    set: HashSet<T>,
}

impl<T: Eq + Hash + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self {
            set: input.iter().cloned().collect(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.set.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.is_subset(&other.set)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.set.is_disjoint(&other.set)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            set: self.set.intersection(&other.set).cloned().collect(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            set: self.set.difference(&other.set).cloned().collect(),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            set: self.set.union(&other.set).cloned().collect(),
        }
    }
}
