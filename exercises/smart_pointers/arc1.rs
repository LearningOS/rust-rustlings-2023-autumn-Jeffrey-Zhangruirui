// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

#![forbid(unused_imports)]
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers); // Arc to share the vector among threads
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let shared_numbers_clone = Arc::clone(&shared_numbers); // Clone for each thread
        joinhandles.push(thread::spawn(move || {
            let child_numbers: Vec<_> = shared_numbers_clone
                .iter()
                .cloned()
                .filter(|&n| n % 8 == offset)
                .collect();

            let sum: u32 = child_numbers.iter().sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }

    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
