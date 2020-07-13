use crate::models::student::{Student, StudentType};
use crate::models::class::Class;

mod models;

fn main() {
    println!("classes exercise!!");
    let s = Student{
        name: "Daniel Fernando".to_string(),
        high_school: "IPARM".to_string(),
        program: "".to_string(),
        student_type: StudentType::HighSchool,
        grade: 'A',
    };
    let s1 = Student{
        name: "Daniel".to_string(),
        high_school: "IPARM".to_string(),
        program: "".to_string(),
        student_type: StudentType::HighSchool,
        grade: 'A',
    };
    let s2 = Student{
        name: "Fernando".to_string(),
        high_school: "CCFDV".to_string(),
        program: "".to_string(),
        student_type: StudentType::HighSchool,
        grade: 'F',
    };
    let s3 = Student{
        name: "Fernando".to_string(),
        high_school: "CCFDV".to_string(),
        program: "".to_string(),
        student_type: StudentType::HighSchool,
        grade: 'F',
    };
    assert_eq!(s2,s3);
    let mut vec = Vec::new();
    vec.push(s1);
    vec.push(s2);
    assert_eq!(2,vec.len());

    let mut class = Class{
        name: "Backend".to_string(),
        professor_name: "Dafer".to_string(),
        students: vec![]
    };
    class.enroll(s);
    class.enroll_batch(vec);

    println!("class name: {}, class # students {}", class.name, class.students.len());
}
