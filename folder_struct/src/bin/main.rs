use std::iter::FromIterator;
use folder_struct::students::high_school::HighSchoolStudent;
use folder_struct::students::college::CollegeStudent;
use folder_struct::class::Class;

fn main() {
    let hs_class_students: Vec<&HighSchoolStudent> = Vec::new();
    let hs_class_students1: Vec<&HighSchoolStudent> = Vec::new();
    let college_class_students: Vec<&CollegeStudent> = Vec::new();

    let mut hs_class: Class<HighSchoolStudent> = Class::new(
        "biology".to_string(),
        "charles darwin".to_string(), hs_class_students,
    );
    let mut college_class: Class<CollegeStudent> = Class::new(
        "computing sciences".to_string(),
        "dijkstra".to_string(), college_class_students,
    );
    let mut hs_class1: Class<HighSchoolStudent> = Class::new(
        "maths".to_string(),
        "whatever".to_string(), hs_class_students1,
    );

    let mut student = HighSchoolStudent::new(3.6, "oscar".to_string(), "hs 1".to_string());
    hs_class.enroll_student(&mut student);

    let mut student1 = HighSchoolStudent::new(4.5, "oscar1".to_string(), "hs 1".to_string());
    hs_class.enroll_student(&mut student1);

    let mut student2 = HighSchoolStudent::new(3.55, "ivan".to_string(), "hs 2".to_string());
    hs_class.enroll_student(&mut student2);

    let mut student3 = CollegeStudent::new(3.6, "gutierrez".to_string(), "systems eng".to_string());
    college_class.enroll_student(&mut student3);

    let mut student4 = CollegeStudent::new(3.55, "rincon".to_string(), "architecture".to_string());
    college_class.enroll_student(&mut student4);

    let mut student5 = CollegeStudent::new(5.0, "rincon1".to_string(), "architecture".to_string());
    college_class.enroll_student(&mut student5);

    let mut student6 = HighSchoolStudent::new(3.6, "oscar".to_string(), "hs 1".to_string());
    hs_class1.enroll_student(&mut student6);

    let mut student7 = HighSchoolStudent::new(4.5, "oscar1".to_string(), "hs 1".to_string());
    hs_class1.enroll_student(&mut student7);

    assert!(!college_class.all_on_same());
    assert!(!hs_class.all_on_same());
    assert!(hs_class1.all_on_same());

    println!("{:#?}", college_class.rank_students());
    println!("{:#?}", hs_class.rank_students());

    println!("");
    println!("");
    println!("");
    println!("");
    println!("");

    println!("{:#?}", college_class);
    println!("{:#?}", hs_class);

    assert!(student4 == student5);
    assert!(!(student3 == student5));
}


