// Topic: Option
//
// Requirements:

// Notes:
// * Lockers use numbers and are optional for students

// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    // * The locker assignment should use an Option<i32>
    locker: Option<i32> 
}

fn main() {
    let created_student = Student {
        name: String::from("Uchechukwu"),
        locker: None
    };
    
    // * Print out the details of a student's locker assignment
    match created_student.locker {
        Some(locker) => println!("Locker Details: {:?}", locker),
        None => println!("No locker detail available")
    }
}


