// tests5.rs
//
//Rust中的“unsafe”充当合约。
//
//当在诸如函数之类的项目声明上标记“不安全”时，
//如果它具有某种特征等，它会在旁边声明一份合同。然而，
//合同的内容不能仅由一个关键字来表达。
// 因此，您有责任在 `# Safety`
// 文档中关于该项目的注释部分。
//
//当在花括号括起来的代码块上标记“unsafe”时，
//它声明遵守某些合同，例如某些合同的有效性
//指针参数，某个内存地址的所有权。然而，就像
//在上面的文本中，您仍然需要说明如何遵守合同
//代码块上的注释。
//
//注：所有注释都是为了可读性和可维护性
//您的代码，而Rust编译器则信任您的代码的可靠性
//代码给自己！如果你不能证明记忆的安全性和可靠性
//你自己的代码，退一步，使用安全代码！
//
//执行`rustlings hint tests5`或使用`hint` watch子命令
//提示。


/// # Safety
///
///“address”必须包含对有效“u32”值的可变引用。
unsafe fn modify_by_address(mut address: usize) {
    //TODO:填写下面代码块的安全通知，以匹配您的
    //代码的行为和此函数的契约。您可以使用
    //下面的测试注释作为您的格式参考。
    unsafe {
        let mut p = address as *mut u32;
        *p = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        //安全：地址保证有效，并包含
        //对“u32”局部变量的唯一引用。
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD);
    }
}
