// errors2.rs
//

//假设我们正在编写一个游戏，你可以用代币购买物品。所有项目成本
//5个代币，每当您购买物品时，都需要支付1的手续费
//令牌。游戏玩家将键入他们想购买的物品数量，然后
//`total_cost`函数将计算令牌的总成本。自从
//不过，玩家输入了数量，我们得到的是字符串——他们
//可能输入了任何东西，而不仅仅是数字！
//
//现在，此函数根本不处理错误情况（也不是
//正确处理成功案例）。我们想做的是：如果我们打电话
//对于非数字字符串的`parse`函数，该函数将
//返回一个`ParseIntError`，在这种情况下，我们希望立即返回
//这个错误来自我们的函数，不要尝试乘法和加法。

//至少有两种方法都是正确的，但有一种
//要短得多！
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();
    match qty {
        Err(_) => qty,
        Ok(qty) => Ok(qty * cost_per_item + processing_fee)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
