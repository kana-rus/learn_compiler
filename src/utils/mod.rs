use std::{fmt::Display ,collections::vec_deque::VecDeque};

pub type Int = u32;


pub fn assert(condition: bool, message: &str) {
    if !condition {
        println!("{}", message);
        panic!();
    }
}
pub fn exit_with_report<D: Display>(msg: D) -> ! {
    println!("{}", msg);
    panic!()
}
pub fn build_from(degits: &VecDeque<Int>) -> Int {
    degits.iter().fold(0, |a, b| 10*a + b)
}




