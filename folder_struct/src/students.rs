pub mod college;
pub mod high_school;


#[derive(Debug)]
pub struct Student {
    name: String,
    grade: f32
}

impl Student {
    pub fn new(grade: f32, name: String) -> Self {
        Student {
            grade,
            name
        }
    }

    pub fn grade(&self) -> f32 {
        self.grade
    }
}


