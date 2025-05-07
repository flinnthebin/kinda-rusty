const MAX: i32 = 100;
static mut COUNT: i32 = 0;

#[allow(static_mut_refs)]
pub fn demo() {
    let x = 10;
    let y = x as f64;
    let flag = true;
    let done = false;
    println!("x={} y={} MAX={} flag={} done={}", x, y, MAX, flag, done);
    unsafe {
        COUNT = 1;
        println!("COUNT={}", COUNT);
    }
}
