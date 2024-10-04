// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.


//此函数返回冰箱中还剩多少冰淇淋。
//如果是晚上10点之前，还剩5块。晚上10点，有人吃了它们
//所有，所以不会再有了：(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    //我们在这里使用24小时制，所以晚上10点是22，上午12点是
    //值0选项输出应优雅地处理以下情况
    //一天中的时间>23。
    //TODO:完成函数体-记得返回一个Option！
    if time_of_day < 22 {
        Some(5)
    } else if time_of_day < 24 {
        Some(0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
