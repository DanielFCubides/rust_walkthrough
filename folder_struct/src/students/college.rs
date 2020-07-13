use crate::students::Student as Student;

#[derive(Debug)]
pub struct CollegeStudent {
    student: Student,
    program: String
}

impl CollegeStudent {
    pub fn new(grade: f32, name: String, program: String) -> CollegeStudent {
        CollegeStudent {
            student: Student::new(grade, name),
            program
        }
    }
}

impl PartialEq for CollegeStudent {
    fn eq(&self, other: &Self) -> bool {
        self.program == other.program
    }
}

impl Eq for CollegeStudent{}

impl PartialOrd for CollegeStudent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.student.grade().partial_cmp(&other.student.grade())
    }
}

impl Ord for CollegeStudent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.student.grade().partial_cmp(&other.student.grade()) {
            Some(v) => v,
            None => panic!("bad element")
        }
    }
}