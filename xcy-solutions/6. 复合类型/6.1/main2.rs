
// 使用至少两种方法来修复错误
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(s)
// }
//
// fn greetings(s: &str) {
//     println!("{}",s)
// }


// 使用至少两种方法来修复错误
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
    greetings2(s);
    let s: Box<&str> = "hello, world".into();
    greetings3(*s);
}

fn greetings(s: &str) {
    println!("{}",s)
}

fn greetings2(s: Box<str>) {
    println!("{}",s)
}

fn greetings3(s: &str) {
    println!("{}",s)
}

