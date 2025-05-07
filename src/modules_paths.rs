mod outer {
    pub fn parent() {
        println!("in parent");
    }
    pub mod inner {
        pub fn call_parent() {
            super::parent();
        }
    }
}

use outer::inner::call_parent;

pub fn parentcall() {
    call_parent();
}
