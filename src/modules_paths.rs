// parent module
mod outer {
    pub fn parent() {
        println!("in parent");
    }
    // child module
    pub mod inner {
        // inheritance
        pub fn call_parent() {
            super::parent();
        }
    }
}
// import function into scope
use outer::inner::call_parent;

// wrapper
pub fn parentcall() {
    call_parent();
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        parentcall();
    }
}
