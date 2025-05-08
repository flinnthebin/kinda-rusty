// bounded trait definition
pub trait Speak
where
    // restrict to types that implement debug
    Self: std::fmt::Debug,
{
    fn speak(&self); // signature for speaking
}
// derive debug to satisfy trait bound & print with {:?}
#[derive(Debug)]
pub struct Person {
    pub name: String,
}
// implement speak trait for person
impl Speak for Person {
    fn speak(&self) {
        // use debug formatting for self.name when printing
        println!("{:?} says hello", self.name);
    }
}
// dynamic dispatch helper - accept any speak implementor behind a trait object & call speak()
pub fn speak_dyn(s: &dyn Speak) {
    s.speak();
}
