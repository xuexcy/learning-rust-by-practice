//
// use std::num::ParseIntError;
//
// // 使用 `?` 来实现 multiply
// // 不要使用 unwrap !
// fn multiply(n1_str: &str, n2_str: &str) -> __ {
// }
//
// fn main() {
//     assert_eq!(multiply("3", "4").unwrap(), 12);
//     println!("Success!")
// }


use std::num::ParseIntError;

// 使用 `?` 来实现 multiply
// 不要使用 unwrap !
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    Ok(n1_str.parse::<i32>()? * n2_str.parse::<i32>()?)
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!")
}
