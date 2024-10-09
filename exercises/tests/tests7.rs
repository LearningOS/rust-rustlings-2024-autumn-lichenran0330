// tests7.rs
//
//构建包时，某些依赖项既不能导入
//`Cargo.toml`也不能直接链接；一些预处理因代码而异
//生成以设置特定于软件包的配置。
//
//Cargo并不打算取代其他构建工具，但它确实集成了
//使用名为“build.rs”的自定义构建脚本。此文件是
//通常放在项目的根目录中，而在这种情况下也是如此
//本练习的目录。
//
//它可用于：
//
//-构建一个捆绑的C库。
//-在主机系统上查找C库。
//-从规范生成Rust模块。
//-执行机箱所需的任何平台特定配置。
//
//在设置配置时，我们可以使用`println！`在构建脚本中
//告诉Cargo遵守一些指示。通用格式为：
//
//println！（“货物：｛｝”，your_command_in_string）；
//
//请参阅关于构建脚本的官方Cargo书籍以了解更多信息
//信息：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
//在这个练习中，我们寻找一个环境变量，并期望它
//落在一个范围内。您可以查看测试用例以了解详细信息。
//
//您不应该修改此文件。修改同一目录中的`build.rs`
//通过这个练习。
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
