// tests8.rs
//
//这个execrise与前一个练习共享“build.rs”。
//您需要在`build.rs`中添加一些代码，以便进行此练习和
//前一项工作。
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
