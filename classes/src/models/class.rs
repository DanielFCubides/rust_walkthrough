use crate::models::student::{Student, StudentType};

pub struct Class {
    pub(crate) name: String,
    pub(crate) professor_name: String,
    pub(crate) students: Vec<Student>,
}


impl Class {
    pub(crate) fn enroll(&mut self, student : Student){
        if self.students.len() == 0 {
            self.students.push(student);
            return
        }
        if self.students[0].student_type == student.student_type {
            self.students.push(student);
            return
        }
    }

    pub(crate) fn enroll_batch(&mut self, students : Vec<Student>){
        students.into_iter()
            .for_each(|s| self.enroll(s))
    }
}