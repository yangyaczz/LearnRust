use std::collections::{HashMap, HashSet};

// student struct
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

// club struct
pub struct Club {
    id: u32,
    name: String,
    members: HashSet<u32>,
}

// class struct
pub struct Class {
    id: u32,
    name: String,
    students: HashSet<u32>,
}

// cource struct
pub struct Course {
    id: u32,
    name: String,
    students_enrolled: HashSet<u32>,
}

// StudentManagementSystem struct
pub struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    clubs: HashMap<u32, Club>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
}

impl StudentManagementSystem {
    // 构造函数
    pub fn new() -> Self {
        StudentManagementSystem {
            students: HashMap::new(),
            clubs: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    pub fn add_club(&mut self, club: Club) {
        self.clubs.insert(club.id, club);
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.id, class);
    }

    pub fn add_course(&mut self, course: Course) {
        self.courses.insert(course.id, course);
    }

    pub fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students.get(&student_id)
    }

    pub fn get_club(&self, club_id: u32) -> Option<&Club> {
        self.clubs.get(&club_id)
    }

    pub fn get_class(&self, class_id: u32) -> Option<&Class> {
        self.classes.get(&class_id)
    }

    pub fn get_course(&self, course_id: u32) -> Option<&Course> {
        self.courses.get(&course_id)
    }

    pub fn add_student_to_club(
        &mut self,
        student_id: u32,
        club_id: u32,
    ) -> Result<(), &'static str> {
        if let Some(student) = self.students.get_mut(&student_id) {
            if let Some(club) = self.clubs.get_mut(&club_id) {
                club.members.insert(student_id);
                Ok(())
            } else {
                Err("Club not found")
            }
        } else {
            Err("Student not found")
        }
    }

    pub fn remove_student_from_club(
        &mut self,
        student_id: u32,
        club_id: u32,
    ) -> Result<(), &'static str> {
        if let Some(student) = self.students.get_mut(&student_id) {
            if let Some(club) = self.clubs.get_mut(&club_id) {
                club.members.remove(&student_id);
                Ok(())
            } else {
                Err("Club not found")
            }
        } else {
            Err("Student not found")
        }
    }
}
