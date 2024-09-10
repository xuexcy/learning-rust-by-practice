// // 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
// use std::str::FromStr;
// fn main() {
//     let parsed: i32 = "5".__.unwrap();
//     let turbo_parsed = "10".__.unwrap();
//     let from_str = __.unwrap();
//     let sum = parsed + turbo_parsed + from_str;
//     assert_eq!(sum, 35);
//
//     println!("Success!")
// }

// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse::<i32>().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = "20".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}
