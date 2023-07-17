// 学生
#[derive(Debug)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub class_id: u32,
}

// 社团
#[derive(Debug)]
pub struct Club {
    pub id: u32,
    pub name: String,
    pub member_ids: Vec<u32>,
}

// 班级
#[derive(Debug)]
pub struct Class {
    pub id: u32,
    pub name: String,
    pub student_ids: Vec<u32>,
    pub course_ids: Vec<u32>,
}

// 课程
#[derive(Debug, Clone)]
pub struct Course {
    pub id: u32,
    pub name: String,
}

// 学生管理系统
#[derive(Debug)]
pub struct StudentManageSystem {
    pub students: Vec<Student>,
    pub clubs: Vec<Club>,
    pub courses: Vec<Course>,
    pub classes: Vec<Class>,
}

impl StudentManageSystem {
    pub fn new() -> Self {
        StudentManageSystem {
            students: Vec::new(),
            clubs: Vec::new(),
            courses: Vec::new(),
            classes: Vec::new(),
        }
    }
    pub fn add_course(&mut self, course: Course) {
        self.courses.push(course);
    }
    pub fn add_club(&mut self, club: Club) {
        self.clubs.push(club);
    }
    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }
    // 检查课程ID是否存在
    pub fn check_courses(&self, course_id: u32) -> Option<bool> {
        match self.courses.iter().find(|c| c.id == course_id) {
            Some(_) => Some(true),
            None => {
                println!("该课程ID不存在:{}！", course_id);
                None
            }
        }
    }
    // 检查学生id是否存在
    pub fn check_students(&self, student_id: u32) -> Option<bool> {
        match self.students.iter().find(|c| c.id == student_id) {
            Some(_) => Some(true),
            None => {
                println!("该学生ID不存在:{}！", student_id);
                None
            }
        }
    }

    // 将课程添加到班级
    pub fn add_course_to_class(&mut self, class_id: u32, course_id: u32) -> bool {
        if !self.check_courses(course_id).unwrap_or(false) {
            return false;
        }
        if let Some(class) = self.classes.iter_mut().find(|c| c.id == class_id) {
            if !(class.course_ids.contains(&course_id)) {
                class.course_ids.push(course_id);
                println!("该课程已添加到班级:{}！", class_id);
                true
            } else {
                false
            }
        } else {
            println!("该班级ID不存在:{}！", class_id);
            false
        }
    }

    // 调班
    pub fn update_students_to_class(&mut self, class_id: u32, student_id: u32) -> bool {
        if let Some(class) = self.classes.iter_mut().find(|c| c.id == class_id) {
            if let Some(student) = self.students.iter_mut().find(|s| s.id == student_id) {
                if !(class.student_ids.contains(&student_id)) {
                    class.student_ids.push(student_id);
                    student.class_id = class_id;
                    println!("该学生已添加到班级:{}！", class_id);
                    true
                } else {
                    println!("该学生已在班级中存在:{}！", class_id);
                    false
                }
            } else {
                println!("该学生ID不存在:{}！", class_id);
                false
            }
        } else {
            println!("该班级ID不存在:{}！", class_id);
            false
        }
    }

    // 学生加入社团
    pub fn add_students_to_club(&mut self, club_id: u32, student_id: u32) -> bool {
        if !self.check_students(student_id).unwrap_or(false) {
            return false;
        }
        if let Some(club) = self.clubs.iter_mut().find(|c| c.id == club_id) {
            if !(club.member_ids.contains(&student_id)) {
                club.member_ids.push(student_id);
                println!("该学生已加入社团:{}！", club_id);
                true
            } else {
                println!("该学生已在社团中:{}！", club_id);
                false
            }
        } else {
            println!("该社团ID不存在:{}！", club_id);
            false
        }
    }

    // 学生退出社团
    pub fn delete_students_from_club(&mut self, club_id: u32, student_id: u32) -> bool {
        if !self.check_students(student_id).unwrap_or(false) {
            return false;
        }
        if let Some(club) = self.clubs.iter_mut().find(|c| c.id == club_id) {
            if club.member_ids.contains(&student_id) {
                if let Some(index) = club.member_ids.iter().position(|item| item == &student_id) {
                    club.member_ids.remove(index);
                    println!("该学生已加入社团:{}！", club_id);
                    true
                } else {
                    false
                }
            } else {
                println!("该学生ID不在社团中:{}！", club_id);
                false
            }
        } else {
            println!("该社团ID不存在:{}！", club_id);
            false
        }
    }
    // 退出所有社团
    pub fn remove_student_from_clubs(&mut self, student_id: u32) {
        for club in &mut self.clubs {
            if let Some(index) = club.member_ids.iter().position(|s| s == &student_id) {
                club.member_ids.remove(index);
            }
        }
        println!("该学生退出所有社团！")
    }
    // 退出所有班级
    pub fn remove_student_from_classes(&mut self, student_id: u32) {
        for class in &mut self.classes {
            if let Some(index) = class.student_ids.iter().position(|s| s == &student_id) {
                class.student_ids.remove(index);
            }
        }
    }

    // 查询学生所有班级
    // 查询学生报名的社团

    pub fn add_students(&mut self, student: Student) {
        self.students.push(student);
    }
    // 学生退学
    pub fn remove_students(&mut self, student_id: u32) -> bool {
        if !self.check_students(student_id).unwrap_or(false) {
            return false;
        }
        self.remove_student_from_classes(student_id);
        self.remove_student_from_clubs(student_id);

        if let Some(index) = self.students.iter().position(|s| s.id == student_id) {
            self.students.remove(index);
            println!("该学生退学成功: {}", student_id);
            true
        } else {
            println!("students removed failly: {}", student_id);
            false
        }
    }
    pub fn update_student_name(&mut self, id: u32, new_name: String) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = new_name;
            println!("students removed successfully: {}", id);
            true
        } else {
            println!("students removed successfully: {}", id);
            false
        }
    }
    // 查询学生信息
    pub fn get_student(&self, student_id: u32) -> Option<&Student> {
        if !self.check_students(student_id).unwrap_or(false) {
            return None;
        }
        let mut club_vec = Vec::new();
        for club in &self.clubs {
            if club.member_ids.contains(&student_id) {
                club_vec.push(&club.name);
            }
        }
        let club_str = club_vec
            .into_iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(",");
        let mut class_vec = Vec::new();
        for class in &self.classes {
            if class.student_ids.contains(&student_id) {
                class_vec.push(&class.name);
            }
        }
        let class_str = class_vec
            .into_iter()
            .map(|c| c.as_str())
            .collect::<Vec<&str>>()
            .join(",");

        let student = self.students.iter().find(|s| s.id == student_id).unwrap();
        println!(
            "学生：{:?}，在{:?}班级，并加入了{:?}社团",
            &student.name, class_str, club_str
        );
        Some(&student)
    }
}
