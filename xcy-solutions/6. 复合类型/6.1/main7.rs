
// 使用至少两种方法来修复错误
// fn main() {
//     let s = "hello, world";
//     greetings(s)
// }
//
// fn greetings(s: String) {
//     println!("{}",s)
// }


// 使用至少两种方法来修复错误
fn main() {
    let s = "hello, world";
    greetings(s);
    greetings2(s.to_string());
}

fn greetings(s: &str) {
    println!("{}",s)
}

fn greetings2(s: String) {
    println!("{}",s)
}
