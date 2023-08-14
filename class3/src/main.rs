mod student_management_system;

use student_management_system::{StudentManagementSystem, Student, Club, Class, Course};


fn main() {
    let mut system = StudentManagementSystem::new();

    // crud
    let student = Student {
        id: 0,
        name: String::from("yycz"),
        age: 20,
    };
    system.add_student(student)

}
