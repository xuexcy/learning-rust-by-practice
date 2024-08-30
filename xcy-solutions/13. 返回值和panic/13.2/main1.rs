//
// // 填空并修复错误
// use std::num::ParseIntError;
//
// fn multiply(n1_str: &str, n2_str: &str) -> __ {
//     let n1 = n1_str.parse::<i32>();
//     let n2 = n2_str.parse::<i32>();
//     Ok(n1.unwrap() * n2.unwrap())
// }
//
// fn main() {
//     let result = multiply("10", "2");
//     assert_eq!(result, __);
//
//     let result = multiply("t", "2");
//     assert_eq!(result.__, 8);
//
//     println!("Success!")
// }


// 填空并修复错误
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("t", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!")
}
