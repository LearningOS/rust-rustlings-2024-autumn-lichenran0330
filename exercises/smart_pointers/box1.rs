// box1.rs
//
//在编译时，Rust需要知道一个类型占用了多少空间。这个
//对于递归类型来说是有问题的，其中值可以作为
//它本身是同一类型的另一个值。为了解决这个问题，我们可以使用
//“Box”-一个用于在堆上存储数据的智能指针，它也允许我们
//包装递归类型。
//
//我们在这个练习中实现的递归类型是“cons-list”-a
//函数式编程语言中常见的数据结构。每个
//cons列表中的项包含两个元素：当前项的值和
//下一个项目。最后一项是一个名为“Nil”的值。
//
//步骤1：在枚举定义中使用“Box”使代码编译
//步骤2：通过替换`todo！()`
//
//注意：测试不应更改
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.


#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(3, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
