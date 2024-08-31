// // in src/main.rs
//
// // 填空并修复错误
// fn main() {
//     assert_eq!(__, "sit down please");
//     assert_eq!(__,"yummy yummy!");
// }

// in src/main.rs

mod front_of_house;

// 填空并修复错误
fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package4::eat_at_restaurant(),"yummy yummy!");
}
