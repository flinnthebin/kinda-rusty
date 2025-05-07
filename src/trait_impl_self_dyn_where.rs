pub trait Speak
where
    Self: std::fmt::Debug,
{
    fn speak(&self);
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
}

impl Speak for Person {
    fn speak(&self) {
        println!("{:?} says hello", self.name);
    }
}

pub fn speak_dyn(s: &dyn Speak) {
    s.speak();
}
