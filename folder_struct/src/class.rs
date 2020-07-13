use std::iter::FromIterator;

#[derive(Debug)]
pub struct Class<'a, T> {
    name: String,
    professor: String,
    students: Vec<&'a T>
}

impl<'a, T> Class<'a, T> where T: PartialEq + Eq + PartialOrd + Ord {
    pub fn new(name: String, professor: String, students: Vec<&'a T>) -> Self {
        Class {
            name,
            professor,
            students
        }
    }
    pub fn enroll_student(&mut self, student: &'a mut T) {
        self.students.push(student);
    }

    pub fn all_on_same(&self) -> bool {
        self.students.iter().filter(|&x| &self.students[0] != x).count() == 0
    }

    pub fn rank_students(&mut self) -> Vec<&T> {
        // let mut var = self.students.clone();
        // let mut var: Vec<_> = self.students.iter().map(|&x| x).collect();
        let mut var = Vec::from_iter(self.students.iter().cloned());
        var.sort_by(|a, b| b.cmp(a));
        var
    }
}
