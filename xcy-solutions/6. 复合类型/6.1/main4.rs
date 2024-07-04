

// 修复所有错误，并且不要新增代码行
// fn main() {
//     let  s = String::from("hello");
//     s.push(',');
//     s.push(" world");
//     s += "!".to_string();
//
//     println!("{}", s)
// }
// 修复所有错误，并且不要新增代码行
fn main() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}
