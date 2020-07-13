#[derive(PartialEq)]
#[derive(Debug)]
pub enum StudentType {
    HighSchool,
    College,
}
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Student {
    pub(crate) name: String,
    pub(crate) high_school: String,
    pub(crate) program: String,
    pub(crate) student_type: StudentType,
    pub(crate) grade: char,
}

impl Student {
    fn get_type(self) -> StudentType {
        self.student_type
    }

    fn get_name(self) -> String {
        self.name
    }
}