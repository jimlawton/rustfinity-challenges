use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

#[derive(Debug)]
pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: &str) {
        let student = Student {
            name: name.to_string(),
            grades: Vec::<u8>::new(),
        };
        self.students.insert(name.to_string(), student);
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        let mut student: Student = self.students.get(name).unwrap().clone();
        student.grades.push(grade);
        self.students.insert(name.to_string(), student);
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        &self.students.get(name).unwrap().grades
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}

