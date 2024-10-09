// from_into.rs
//
//From特征用于值到值的转换。如果From已实现
//对于一个类型来说，Into特性应该相反。你可以阅读
//更多信息请访问https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

//我们实现了默认特性，将其用作回退
//当提供的字符串不能转换为Person对象时
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}
//你的任务是完成这个实现，以便行`let p=Person：：from（“Mark，20”）`
// 编译请注意，您需要解析
//将年龄成分转换为“usize”，类似于“4”。parse:：<usize>（）`。这个
//这一结果需要妥善处理。
//
//步骤：
// 1.如果提供的字符串的长度为0，则返回默认值
// 2.将给定的字符串拆分为其中的逗号。
// 3.从拆分操作中提取第一个元素并将其用作名称。
// 4.如果名称为空，则返回默认值Person。
// 5.从拆分操作中提取另一个元素，并将其解析为
//随着年龄的增长而“使用”。
//如果在解析年龄时出现问题，则返回默认值
//Person否则，返回一个实例化的Person对象及其结果

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.len() == 0 {
            Person::default()
        } else {
            for i in 0..s.len() {
                if s[i..i+1] == *"," {
                    if i == 0 {
                        return Person::default()
                    } else {
                        match s[i+1..s.len()].parse::<usize>() {
                            Ok(x) => {
                                return Person {
                                    name: s[0..i].to_string(),
                                    age: x
                                }
                            }
                            _ => return Person::default()
                        }
                    }
                }
            }
            Person::default()
        }
    }
}

fn main() {
    //使用“from”函数
    let p1 = Person::from("Mark,20");
    //由于From是为Person实现的，我们应该能够使用Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        //测试默认人是30岁的约翰
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        //测试在提供错误字符串时返回John
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        //测试“Mark，20”是否有效
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        //测试“Mark，twenty”是否会因以下原因返回默认人
        //解析年龄时出错
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
