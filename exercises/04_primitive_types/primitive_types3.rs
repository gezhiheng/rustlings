use std::vec::Vec;

fn main() {
    // : Create an array called `a` with at least 100 elements in it.
    let mut a: Vec<i32> = Vec::with_capacity(101);
    for _ in 0..100 {
        a.push(1);
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
