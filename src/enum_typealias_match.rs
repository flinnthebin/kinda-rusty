// algebraic data type (enum)
pub enum Status {
    Success,
    Failure,
}

// pattern matching & branching
pub fn describe(stat: Status) {
    match stat {
        Status::Success => println!("All good!"),
        Status::Failure => println!("Something went wrong!"),
    }
}

// type alias & local binding
pub fn name() {
    type Name = String;
    let name: Name = "Chris".to_string();
    println!("My name is {}", name);
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        describe(Status::Success);
        describe(Status::Failure);
        name();
    }
}
