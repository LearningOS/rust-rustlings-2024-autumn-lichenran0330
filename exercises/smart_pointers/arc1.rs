// arc1.rs
//
//在这个练习中，我们得到一个u32的Vec，称为“数字”，有值
//范围从0到99-[0,1,2，…，98,99]我们想用这个
//同时在8个不同线程内的一组数字。每个线程都是
//将得到每八个值的总和，并带有偏移量。
//
//第一个线程（偏移量0）的总和为0、8、16。。。
//第二个线程（偏移量1）将加1、9、17。。。
//第三个线程（偏移量2）将加2、10、18。。。
// ...
//第八个线程（偏移量7）将求和7、15、23。。。
//
//因为我们使用线程，所以我们的值需要是线程安全的。因此，
//我们正在使用Arc。我们需要对两个TODO中的每一个进行更改。
//
//通过填写“shared_numbers”的值来编译此代码，其中
//第一个TODO注释是，并为`child_numbers创建初始绑定`
//第二个TODO注释位于何处。请尽量不要创建任何副本
//“数字”维克！
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.


#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
