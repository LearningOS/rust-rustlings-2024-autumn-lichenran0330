// hashmaps1.rs
//
// 需要定义哈希图形式的一篮水果。键表示水果的名称，值表示篮子里有多
// 少特定的水果。你必须在篮子里放至少三种不同类型的水果
// （如苹果、香蕉、芒果），所有水果的总数至少应为五种。


// 让我编译并通过测试！


// 执行`rustlings hint hashmaps1`或使用`hint` watch子命令获取提示。


use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = // TODO: declare your hash map here.
    HashMap::new();
    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert(String::from("apple"), 5);
    basket.insert(String::from("pear"), 5);
    
    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
