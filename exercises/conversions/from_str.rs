// from_str.rs
//
//这类似于from_into.rs，但这次我们将实现`FromStr`和
//返回错误，而不是回退到默认值。此外，在
//实现FromStr时，您可以对字符串使用`parse`方法来生成
//实现者类型的对象。您可以在以下网址阅读更多信息
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

//我们将在`FromStr`实现中使用此错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // 字段数量不正确
    BadLen,
    // 空名称字段
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// Steps:
// 1.如果提供的字符串长度为0，则应返回错误
// 2. 将给定的字符串拆分为其中的逗号
// 3. 拆分后只应返回2个元素，否则返回错误
// 4. 从拆分操作中提取第一个元素并将其用作名称
// 5. 从拆分操作中提取另一个元素，并将其解析为“usize”表示年龄，类似于“4”。parse:：<usize>（）`
// 6. 如果在提取姓名和年龄时出现问题，则会出现错误应该归还
//    如果一切顺利，则返回Person对象的Result
//
// 顺便说一句：`Box<dyn Error>`实现了`From<&'_str>`。这意味着，如果
// 如果您想返回字符串错误消息，只需使用
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {Err(ParsePersonError::Empty)}
        else {
            for i in 0..s.len() {
                if s[i..i+1] == *"," {
                    for j in i+1..s.len() {
                        if s[j..j+1] == *"," {
                            return Err(ParsePersonError::BadLen)
                        }
                    }
                    if i == 0 {
                        return Err(ParsePersonError::NoName)
                    } else {
                        if s.len() == i - 1 {
                            return Err(ParsePersonError::BadLen)
                        }else {
                            match s[i+1..s.len()].parse::<usize>() {
                                Ok(x) => {
                                    return Ok(Person {
                                        name: s[0..i].to_string(),
                                        age: x
                                    })
                                }
                                Err(x) => return Err(ParsePersonError::ParseInt(x))
                            }
                        }
                    }
                    
                }
            }
            Err(ParsePersonError::BadLen)
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
