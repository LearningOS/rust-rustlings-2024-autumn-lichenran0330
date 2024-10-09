// clippy1.rs
//
//Clippy工具是一组用于分析代码的lint，以便您可以
//找出常见错误并改进Rust代码。
//
//对于这些练习，当有clippy时，代码将无法编译
//警告检查clippy输出中的建议以解决练习。
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
