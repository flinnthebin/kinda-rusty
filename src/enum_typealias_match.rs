pub enum Status {
    Success,
    Failure,
}

pub fn describe(stat: Status) {
    match stat {
        Status::Success => println!("All good!"),
        Status::Failure => println!("Something went wrong!"),
    }
}

pub fn name() {
    type Name = String;
    let name: Name = "Chris".to_string();
    println!("My name is {}", name);
}
