// use std::num::ParseIntError;
//
// // 使用两种方式填空: map, and then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
//    n_str.parse::<i32>().__
// }
//
// fn main() {
//     assert_eq!(add_two("4").unwrap(), 6);
//
//     println!("Success!")
// }

use std::num::ParseIntError;

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|x| x + 2)
}
fn add_two_v2(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);
    assert_eq!(add_two_v2("4").unwrap(), 6);

    println!("Success!")
}
