pub fn flow_demo() {
    for i in 0..5 {
        if i % 2 == 0 {
            continue;
        } else {
            println!("odd number: {}", i);
        }
        let mut j = 0;
        while j < 2 {
            println!("j={}", j);
            j += 1;
        }
    }

    loop {
        println!("once");
        break;
    }
}
