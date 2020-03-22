#[macro_use]
extern crate lazy_static;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

const MAX: usize = 100;

lazy_static! {
    static ref UNIQUE: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: unique_random_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = unique_random_name();
    }
}

fn unique_random_name() -> String {
    for _ in 0..MAX {
        let name: String = random_name();
        let mut used = UNIQUE.lock().unwrap();
        if !used.contains(&name) {
            used.insert(name.clone());
            return name;
        }
    }

    panic!("Unable to find an unique name after {} trials.", MAX);
}

fn random_name() -> String {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    (0..2)
        .map(|_| rng1.gen_range(0, 26))
        .map(|c| c + 'A' as u8)
        .chain((0..3).map(|_| rng2.gen_range(0, 10) + '0' as u8))
        .map(|c| c as char)
        .collect()
}
