macro_rules! read_stdin {
    ($t:ty) => {{
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

pub fn reader() {
    let v: Vec<i32> = read_stdin!(i32);
    println!("{:?}", v);
}
