pub fn flow_demo() {
    // for-range loop
    for i in 0..5 {
        // conditional
        if i % 2 == 0 {
            continue;
        } else {
            println!("odd number: {}", i);
        }
        let mut j = 0;
        // while loop
        while j < 2 {
            println!("j={}", j);
            j += 1;
        }
    }
    // infinite loop
    let mut x = 0;
    loop {
        println!("loop");
        // conditional
        if x == 1 {
            break;
        } else {
            x += 1;
        }
    }
}
