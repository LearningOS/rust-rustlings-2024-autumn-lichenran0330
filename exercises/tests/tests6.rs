// tests6.rs
//
//在这个例子中，我们对Rust标准库的
//不安全的功能。修正所有的问号和待办事项，以便进行测试
//通过。
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// “ptr”必须包含一个“Foo”的拥有框。
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    //安全性：`ptr `包含一个由合约拥有的`Foo`盒子。我们
    //只需根据该指针重建该框。
    let mut ret: Box<Foo> = unsafe {
        Box::from_raw(ptr)
    };
    (*ret).b = Some("hello".to_string());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // 安全：我们经过一个自有的“Foo”盒子。
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert_eq!(ptr_1, ptr_2);
        assert_eq!(ret.b, Some("hello".to_owned()));
    }
}
