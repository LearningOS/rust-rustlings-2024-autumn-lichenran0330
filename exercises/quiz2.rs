// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
//让我们以函数的形式构建一个小机器。作为输入，我们将
//给出字符串和命令列表。这些命令决定了什么操作
//将应用于字符串。它可以是：
//-将字符串大写
//-修剪绳子
//-将“bar”附加到字符串指定次数
//具体形式如下：
//-输入将是一个2长度元组的向量，
//第一个元素是字符串，第二个元素是命令。
//-输出元素将是字符串的Vector。
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(x) => {
                    let mut y = *x;
                    let mut a: String = string.to_string();
                    while y > 0 {
                        y -= 1;
                        a.push_str("bar");
                    }
                    output.push(a);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
