
// fn main() {
//     let tup = (1, 6.4, "hello");
//
//     // 填空
//     let __ = tup;
//
//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
// }


fn main() {
    let tup = (1, 6.4, "hello");

    // 填空
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}
