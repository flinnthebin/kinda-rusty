mod async_await;
mod const_cast_unsafe;
mod enum_typealias_match;
mod extern_c_ffi;
mod loops_branching;
mod modules_paths;
mod move_semantics_threads;
mod practice;
mod trait_impl_self_dyn_where;

fn main() {
    println!("----------\nExercise 1\n----------");
    use crate::const_cast_unsafe;
    const_cast_unsafe::demo();
    println!("----------\nExercise 2\n----------");
    use crate::loops_branching;
    loops_branching::flow_demo();
    println!("----------\nExercise 3\n----------");
    use crate::enum_typealias_match;
    enum_typealias_match::describe(enum_typealias_match::Status::Success);
    enum_typealias_match::describe(enum_typealias_match::Status::Failure);
    enum_typealias_match::name();
    println!("----------\nExercise 4\n----------");
    use crate::trait_impl_self_dyn_where::*;
    let p = Person {
        name: "Chris".into(),
    };
    p.speak();
    speak_dyn(&p);
    println!("----------\nExercise 5\n----------");
    use crate::modules_paths;
    modules_paths::parentcall();
    println!("----------\nExercise 6\n----------");
    use crate::async_await;
    async_await::asynchronous();
    println!("----------\nExercise 7\n----------");
    use crate::extern_c_ffi;
    extern_c_ffi::absval();
    println!("----------\nExercise 8\n----------");
    use crate::move_semantics_threads;
    move_semantics_threads::move_thread();
}
