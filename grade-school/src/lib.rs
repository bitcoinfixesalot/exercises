use std::collections::HashMap;

pub struct School {
    grades_students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades_students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades_students
            .entry(grade)
            .or_insert(Vec::new())
            .push(student.to_string())
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.grades_students.keys().map(|g| *g).collect();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades_students.get(&grade).map(|g| {
            let mut students = g.clone();
            students.sort();
            students
        })
    }
}
