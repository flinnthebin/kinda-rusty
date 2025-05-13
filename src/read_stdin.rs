#[allow(unused_macros)]
macro_rules! read_stdin {
    ($t:ty) => {{
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
            .split_whitespace()
            .map(|s| s.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

#[allow(unused_macros)]
macro_rules! read_lines {
    () => {{
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        let lines = stdin.lock().lines();
        lines.map(|l| l.unwrap()).collect::<Vec<String>>()
    }};
}

#[allow(unused_macros)]
macro_rules! read_triples {
    ($t1:ty, $t2:ty, $t3:ty) => {{
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let mut v = Vec::new();
        while let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) {
            v.push((
                x.parse::<$t1>().unwrap(),
                y.parse::<$t2>().unwrap(),
                z.parse::<$t3>().unwrap(),
            ));
        }
        v
    }};
}
pub fn reader() {
    let v: Vec<(i32, i32, i32)> = read_triples!(i32, i32, i32);
    println!("{:?}", v);
}
