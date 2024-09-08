//
// /* 使用两种方法填空 */
// fn main() {
//     __;
//     need_static(v);
//
//     println!("Success!")
// }
//
// fn need_static(r : &'static str) {
//     assert_eq!(r, "hello");
// }


/* 使用两种方法填空 */
fn main() {
    let v = "hello";
    need_static(v);
    let v: &'static str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}
