pub fn vec_ops() {
    let mut v = vec![1, 2, 3]; // creation
    println!("creation  {:?}", v);
    v.push(4); // push
    println!("push 4: {:?}", v);
    v.pop();
    println!("pop: {:?}", v);
    v.extend([5, 6].iter().cloned()); // extend
    println!("extend: {:?}", v);
    let mut w = vec![7, 8];
    v.append(&mut w); // concatenate
    println!("append w: {:?}", v);
    v.retain(|&x| x % 2 == 0); // keep even values
    println!("retain evens: {:?}", v);
    v.clear();
    println!("clear: {:?}", v);
    v.extend([3, 1, 2, 4].iter().cloned());
    println!("extend: {:?}", v);
    v.sort();
    println!("sorted: {:?}", v);
    println!("first: {}, last: {}", v[0], v[v.len() - 1]);
    for x in &v {
        println!("iter val: {}", x);
    }
}
