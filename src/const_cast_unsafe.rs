const MAX: i32 = 100; // compile-time constant
static mut COUNT: i32 = 0; // mutable static reqs `unsafe` to read or write

#[allow(static_mut_refs)] // lint attribute
pub fn demo() {
    let x = 10; // binding
    let y = x as f64; // casting
    let flag = true;
    let done = false;
    println!("x={} y={} MAX={} flag={} done={}", x, y, MAX, flag, done);
    unsafe {
        // unsafe block (static mut)
        COUNT = 1;
        println!("COUNT={}", COUNT);
    }
}
