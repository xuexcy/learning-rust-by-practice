
// 移除某个部分让代码工作
fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // 这里 z 的类型是?
    // z type: i32
    println!("z type: {}", type_of(&z));
}

fn type_of<T>(_: &T) -> String {
    // println!("{}", std::any::type_name::<T>());
    format!("{}", std::any::type_name::<T>())
}
