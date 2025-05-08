// bring the module into scope
use std::thread;

pub fn move_thread() {
    // create heap-allocated vector & move it into the new thread
    let data = vec![1, 2, 3];
    // `move || {..}` takes ownership of `data`
    let handle = thread::spawn(move || {
        // in new thread, print moved-in data
        println!("data = {:?}", data);
    });
    // `join()` blocks until spawned thread finishes, returning a Result
    // `unwrap()` panics if thread panicked, else returns ()
    handle.join().unwrap();
}
