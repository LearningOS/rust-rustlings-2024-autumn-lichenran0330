// tests9.rs
//
//Rust非常能够与C/C++和其他静态编译的语言共享FFI接口
//语言，它甚至可以在代码本身中链接！它通过了外部
//块，就像下面的代码一样。
//
//“extern”关键字后的短字符串表示外部导入的是哪个ABI
//功能将随之而来。在这个练习中，使用了“Rust”，而其他变体则存在，如
//“C”代表标准C ABI，“stdcall”代表Windows ABI。
//
//外部导入的函数在extern块中声明，用分号表示
//标记签名的末尾，而不是花括号。一些属性可以应用于这些
//用于修改链接行为的函数声明，例如#[link_name=“..”]
//修改实际的符号名称。
//
//如果你想将你的符号导出到链接环境中，`extern`关键字可以
//也可以在函数定义之前用相同的ABI字符串注释标记。默认ABI
//因为Rust函数的字面意思是“Rust”，所以如果你想链接到纯Rust函数，
//可以省略整个外部项。
//
//Rust默认会破坏符号，就像C++一样。抑制此行为并使
//那些可按名称寻址的函数，可以应用属性#[nomangle]。
//
//在本练习中，您的任务是使测试用例能够调用`my_demo_function`
//模块Foo。my_demo_function_alias是my_demo_function的别名，因此两者
//测试用例中的代码行应该调用相同的函数。
//
// You should NOT modify any existing code except for adding two lines of attributes.


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    
    #[link_name="my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
