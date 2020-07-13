use super::Student as Student;

#[derive(Debug)]
pub struct HighSchoolStudent {
    student: Student,
    high_school: String
}

impl HighSchoolStudent {
    pub fn new(grade: f32, name: String, high_school: String) -> Self {
        HighSchoolStudent {
            student: Student::new(grade, name),
            high_school
        }
    }
}

impl PartialEq for HighSchoolStudent {
    fn eq(&self, other: &Self) -> bool {
        self.high_school == other.high_school
    }
}

impl Eq for HighSchoolStudent{}

impl PartialOrd for HighSchoolStudent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.student.grade().partial_cmp(&other.student.grade())
    }
}

impl Ord for HighSchoolStudent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.student.grade().partial_cmp(&other.student.grade()) {
            Some(v) => v,
            None => panic!("bad element")
        }
    }

}