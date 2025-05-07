use std::thread;

pub fn move_thread() {
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("data = {:?}", data);
    });
    handle.join().unwrap();
}
