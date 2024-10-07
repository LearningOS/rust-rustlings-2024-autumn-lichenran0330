// threads1.rs
//
//此程序生成多个线程，每个线程运行至少250毫秒，以及
//每个线程返回它们完成所花费的时间。该计划应
//等待所有生成的线程都完成，并应收集它们的
//将值返回到向量中。
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
