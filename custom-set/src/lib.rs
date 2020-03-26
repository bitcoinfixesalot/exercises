#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    set: Vec<T>,
}

impl<T: Clone + PartialEq + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut custom_set = CustomSet {
            set: input.to_vec(),
        };
        custom_set.set.sort();
        custom_set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.set.contains(&element) {
            self.set.push(element);
            self.set.sort();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.iter().all(|element| other.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.set.iter().any(|element| other.contains(element))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut set = CustomSet { set: Vec::new() };
        for element in self.set.iter() {
            if other.contains(element) {
                set.add(element.clone());
            }
        }
        set
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut set = CustomSet { set: Vec::new() };
        for element in self.set.iter() {
            if !other.contains(element) {
                set.add(element.clone());
            }
        }
        set
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut set = CustomSet { set: Vec::new() };
        for element in self.set.iter().chain(other.set.iter()) {
            set.add(element.clone());
        }
        set
    }
}
