unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn absval() {
    unsafe {
        println!("|-3| = {}", abs(-3));
    }
}
