// The "#[derive(Debug)]" attribute in Rust allows you to automatically generate code that helps you print the values of structs and enums for debugging purposes.

#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    let melvin = Employee {
        position: Position::Manager,
        work_hours: 40,
    };
    let bee = Employee {
        position: Position::Supervisor,
        work_hours: 40,
    };
    println!("{:?}", me);
    println!("{:?}", melvin);
    println!("{:?}", bee);
}
