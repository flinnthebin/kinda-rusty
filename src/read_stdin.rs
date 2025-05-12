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

macro_rules! read_lines {
    () => {{
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        let lines = stdin.lock().lines();
        lines.map(|l| l.unwrap()).collect::<Vec<String>>()
    }};
}

macro_rules! read_tuples {
    ( $( $t:ty ),+ ) => {{
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let mut v = Vec::new();
        while let Some(first) = iter.next() {
            let mut tuple = Vec::new();
            tuple.push(first.parse::<$($t),+>().unwrap());
            for _ in 1..($($t)+) {
                tuple.push(iter.next().unwrap().parse::<$t>().unwrap())
            }
            v.push(tuple);
        }
        v
    }};
}

pub fn reader() {
    let v: Vec<i32> = read_stdin!(i32);
    println!("{:?}", v);
}
