// using_as.rs
//
//Rust中的类型转换是通过使用`as`运算符完成的。请注意
//“as”运算符不仅在类型转换时使用。它也有助于
//重命名导入。
//
//目标是确保该部门不会编译失败
//返回正确的类型。
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.


fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
