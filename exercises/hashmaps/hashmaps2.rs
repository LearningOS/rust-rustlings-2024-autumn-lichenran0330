// hashmaps2.rs
//
//我们正在收集不同的水果来烤一个美味的水果蛋糕。为此，
//key 代表我们收集的每种水果的名称，
//value 我们收集了许多这种特殊的水果
//苹果（4）、芒果（2）和荔枝（5）已经在篮子哈希图中。你
//必须在篮子里加水果，这样每种水果至少有一个
//总共超过11个——我们有很多嘴要喂。你不被允许
//再插入这些水果！
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,  //4
    Banana, 
    Mango,  //2
    Lychee, //5
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        //TODO：如果新水果尚未出现在
        //篮子。请注意，你不允许放任何类型的水果
        //已经存在！
        basket.entry(fruit).or_insert(100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't modify this function!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        for amount in basket.values() {
            assert_ne!(amount, &0);
        }
    }
}
