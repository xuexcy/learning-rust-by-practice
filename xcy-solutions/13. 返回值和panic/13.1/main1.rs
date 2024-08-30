
// // 填空
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // 实现下面的代码
//         __
//      }
//
//     println!("Exercise Failed if printing out this line!");
// }
//
// fn main() {
//     drink(__);
//
//     println!("Exercise Failed if printing out this line!");
// }


// 填空
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        panic!("delay no more, pool guy")
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}
