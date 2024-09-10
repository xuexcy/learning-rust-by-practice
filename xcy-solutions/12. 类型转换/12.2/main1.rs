// fn main() {
//     // impl From<bool> for i32
//     let i1: i32 = false.into();
//     let i2: i32 = i32::from(false);
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);
//
//     // 使用两种方式修复错误
//     // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
//     // 2. 上一章节中介绍过的某个关键字
//     let i3: i32 = 'a'.into();
//
//     // 使用两种方法来解决错误
//     let s: String = 'a' as String;
//
//     println!("Success!")
// }

fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // 使用两种方式修复错误
    // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
    // 2. 上一章节中介绍过的某个关键字
    let i3: u32 = 'a'.into();
    let i3: i32 = 'a' as i32;

    // 使用两种方法来解决错误
    let s: String = 'a'.into();
    let s: String = String::from('a');

    println!("Success!")
}
