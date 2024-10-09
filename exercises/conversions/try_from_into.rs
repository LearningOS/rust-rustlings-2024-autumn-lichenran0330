// try_from_into.rs
//
//TryFrom是一种简单而安全的类型转换，在受控状态下可能会失败
//在某些情况下。基本上，这与From相同。主要
//不同之处在于，这应该返回一个Result类型而不是目标
//类型本身。您可以在以下网址阅读更多信息
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//我们将使用此错误类型进行这些“TryFrom”转换。
#[derive(Debug, PartialEq)]
enum IntoColorError {
    //切片长度不正确
    BadLen,
    //整数转换错误
    IntConversion,
}


//您的任务是完成此实现并返回内部的Ok结果
//类型颜色。您需要为三元组创建一个实现
//整数、三个整数的数组和一个整数切片。

//请注意，元组和数组的实现将在编译时检查
//时间，但切片实现需要检查切片长度！另请注意
//正确的RGB颜色值必须是0..=255范围内的整数。

//双重实施
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        if tuple.0 >= 0
            && tuple.0 <= 255
            && tuple.1 >= 0
            && tuple.1 <= 255
            && tuple.2 >= 0
            && tuple.2 <= 255
        {
            return Ok(Color {
                red: tuple.0 as u8,
                green: tuple.1 as u8,
                blue: tuple.2 as u8,
            });
        }
        Err(IntoColorError::IntConversion)
    }
}

//数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        if arr[0] >= 0
            && arr[0] <= 255
            && arr[1] >= 0
            && arr[1] <= 255
            && arr[2] >= 0
            && arr[2] <= 255
        {
            return Ok(Color {
                red: arr[0] as u8,
                green: arr[1] as u8,
                blue: arr[2] as u8,
            });
        }
        Err(IntoColorError::IntConversion)
    }
}

//切片实施
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        } else {
            if slice[0] >= 0
                && slice[0] <= 255
                && slice[1] >= 0
                && slice[1] <= 255
                && slice[2] >= 0
                && slice[2] <= 255
            {
                return Ok(Color {
                    red: slice[0] as u8,
                    green: slice[1] as u8,
                    blue: slice[2] as u8,
                });
            }
            Err(IntoColorError::IntConversion)
        }
    }
}

fn main() {
    //使用`try_from`函数
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    //由于TryFrom是为Color实现的，我们应该能够使用TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    //对于slice，我们应该使用`try_from`函数
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    //或者将切片放在圆括号内，然后使用TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
