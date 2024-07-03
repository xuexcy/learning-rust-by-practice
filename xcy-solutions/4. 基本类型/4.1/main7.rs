
// 将 ? 替换成你的答案
fn main() {
    let x = 1_000.000_1; // f64
    println!("x : {}", x);
    println!("x type: {}", type_of(&x));
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
