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
    ($t:ty) => {{
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        let lines = stdin.lock().lines();
        lines.map(|l| l.unwrap()).collect::<Vec<String>>()
    }};
}

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
