fn test() {
    let x1 = 42;
    let y1 = Box::new(84);
    {
        let z = (x1, y1);
    }
    let x2 = x1;
    let y2 = y1;
}

#[cfg(test)]
mod test {
    fn testing() {
        use super::test;
        test();
    }
}
