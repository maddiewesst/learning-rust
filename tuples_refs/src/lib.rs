pub struct Student(u32, String, String);

pub fn id(student: &Student) -> u32 {
    Student(student.0.0)
}

pub fn first_name(student: &Student) -> String {
    Student(student.0.1)
}

pub fn last_name(student: &Student) -> String {
    Student(student.0.2)
}