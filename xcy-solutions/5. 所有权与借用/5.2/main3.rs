
// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     borrow_object(s)
// }
//
// fn borrow_object(s: &String) {}


// 修复错误
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s)
}

fn borrow_object(s: &String) {}
