// bounded trait definition
trait Speak
where
    // restrict to types that implement debug
    Self: std::fmt::Debug,
{
    fn speak(&self); // signature for speaking
}
// derive debug to satisfy trait bound & print with {:?}
#[derive(Debug)]
struct Person {
    name: String,
}
// implement speak trait for person
impl Speak for Person {
    fn speak(&self) {
        // use debug formatting for self.name when printing
        println!("{:?} says hello", self.name);
    }
}
// dynamic dispatch helper - accept any speak implementor behind a trait object & call speak()
fn speak_dyn(s: &dyn Speak) {
    s.speak();
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        let p = Person {
            name: "Chris".into(),
        };
        p.speak();
        speak_dyn(&p);
    }
}
