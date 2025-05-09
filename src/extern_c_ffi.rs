// declare external C function using C ABI
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

// safe wrapper
pub fn absval() {
    unsafe {
        // call imported unsafe abs fn
        println!("|-3| = {}", abs(-3));
    }
}
