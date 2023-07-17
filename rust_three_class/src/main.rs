use rust_three_class;

fn main() {
    println!("欢迎来到学生管理系统！");
    let course = rust_three_class::Course {
        id: 1,
        name: String::from("💻"),
    };
    let club = rust_three_class::Club {
        id: 1,
        name: String::from("🧘"),
        member_ids: Vec::new(),
    };
    let class = rust_three_class::Class {
        id: 1,
        name: String::from("一班"),
        student_ids: Vec::new(),
        course_ids: Vec::new(),
    };
    let class_second = rust_three_class::Class {
        id: 2,
        name: String::from("二班"),
        student_ids: Vec::new(),
        course_ids: Vec::new(),
    };
    let mut system = rust_three_class::StudentManageSystem::new();
    system.add_course(course.clone());
    system.add_club(club);
    system.add_class(class);
    system.add_class(class_second);
    system.add_course_to_class(2, 1);
    system.add_course_to_class(1, 1);

    // 添加学生
    let student = rust_three_class::Student {
        id: 1,
        name: String::from("john"),
        class_id: 1,
    };
    system.add_students(student);

    // 修改学生班级
    system.update_students_to_class(2, 1);

    // 学生加入社团
    system.add_students_to_club(1, 1);

    // 获取学生信息
    let s = system.get_student(1);
    println!("学生信息: {:?}", s);

    // 学生退出社团
    system.delete_students_from_club(1, 1);

    // 学生退学
    system.remove_students(1);

    println!("学生管理系统数据:{:#?}", system);
}
