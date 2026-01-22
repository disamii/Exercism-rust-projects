use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone, PartialEq)]
pub struct School {
    roster: HashMap<u32, HashSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        for (_, value) in self.roster.iter() {
            if value.contains(student) {
                return;
            }
        }
        self.roster
            .entry(grade)
            .or_default()
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.roster.keys().cloned().collect();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = self
            .roster
            .get(&grade)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default();

        students.sort();
        students
    }
}
