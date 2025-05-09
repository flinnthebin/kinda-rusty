pub fn vec() {
    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);
}
