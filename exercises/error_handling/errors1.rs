// errors1.rs

//如果您通过，此函数将拒绝生成要打印在姓名标签上的文本
//它是一个空字符串。如果能解释问题所在，那就更好了，
//而不是偶尔返回“无”。幸运的是，Rust也有类似的功能
//构造可用于表示错误条件的“Result”。让我们使用它

// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.


pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
