// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
//
// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T>(x: T, y: T) -> T {
//     x + y
// }

fn main() {
    assert_eq!(sum(1, 2), 3);
    assert_eq!(sum2(1, 2), 3);
}

// 通过两种方法使用特征约束来实现 `fn sum`
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
fn sum2<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>
{
    x + y
}
