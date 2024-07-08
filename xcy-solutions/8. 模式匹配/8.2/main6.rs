
// 修复错误，尽量少地修改代码
// 不要移除任何代码行
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;
//
//     match r {
//        &mut value => value.push_str(" world!")
//     }
// }


// 修复错误，尽量少地修改代码
// 不要移除任何代码行
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    // r: &mut v
    // 如果用 &mut value 去匹配，相当于 &mut v 和 &mut value 匹配, 那么 value 就是 v 的值
    // &mut v / &mut value (像分数约分一样, 把&mut 都约掉)
    // 如果用 value 去匹配, 相等于 &mut v 和 value 匹配, 那么 value 就是 &mut v
    // &mut v / value

    match r {
       // mut value => value.push_str(" world!")
       value => value.push_str(" world!")
    }
    println!("{}", v);
}
