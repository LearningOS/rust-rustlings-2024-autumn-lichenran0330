// as_ref_mut.rs
//
//AsRef和AsMut允许廉价的参考转换。阅读更多
//关于他们https://doc.rust-lang.org/std/convert/trait.AsRef.html以及
// https://doc.rust-lang.org/std/convert/trait.AsMut.html分别。
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.


//获取给定参数中的字节数（不是字符）。
//TODO：适当地添加AsRef特性作为特性绑定。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

//获取给定参数中的字符数（不是字节数）。
//TODO：适当地添加AsRef特性作为特性绑定。
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}


//使用as_mut（）将一个数字平方。
//TODO:添加适当的特征绑定。
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    //TODO：实现函数体
    let x = arg.as_mut();
    *x = *x * *x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
