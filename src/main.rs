// -------- Option ---------

struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        // Using deraf because the student name is behind a ref
        if student.name == *student_name {
            return student.grade;
        }
    }

    None
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(94),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(95),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        },
    ];

    let student_name = String::from("Bob");
    let grade = get_grade(&student_name, &student_db);

    // match grade {
    //     Some(grade) => println!("Grade: {}", grade),
    //     None => println!("Student not found"),
    // }

    // If not interested in both None and Some outcome use if let

    if let Some(g) = grade {
        println!("Grade: {}", g);
    }
}

// Rust does not define null, however we have the Option enum which offers None
// enum Option<T> {
//     Some(T),
//     None
// }
