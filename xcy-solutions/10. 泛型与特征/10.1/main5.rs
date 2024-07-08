
// 为 Val 增加泛型参数，不要修改 `main` 中的代码
// struct Val {
//     val: f64,
// }
//
// impl Val {
//     fn value(&self) -> &f64 {
//         &self.val
//     }
// }
//
//
// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }


// 为 Val 增加泛型参数，不要修改 `main` 中的代码
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T{
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}
