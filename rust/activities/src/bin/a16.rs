// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    /// Locker number is Optional (can be None)
    locker_assignment: Option<i32>,
}
// * The locker assignment should use an Option<i32>

fn main() {
    let becky = Student {
        name: String::from("Becky"),
        locker_assignment: Some(3),
    };

    println!("Student name is:  {:?}", becky.name);

    match becky.locker_assignment {
        Some(num) => {
            println!("Student locker is:  {:?}", num)
        }
        None => println!("no locker_assignment"),
    };
}
