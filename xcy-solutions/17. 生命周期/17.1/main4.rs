// /* 使用三种方法修复下面的错误  */
// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }
//
// fn main() {
// }
//

/* 使用三种方法修复下面的错误  */
fn invalid_output() -> String {
    String::from("foo")
}

fn invalid_output2<'a>() -> &'a str {
    "foo"
}
fn invalid_output3<'a>(s: &'a String) -> &'a String {
    s
}
fn invalid_output4() -> &'static str {
    "foo"
}


fn main() {
}
