// #[derive(Debug)]
// struct Config {
//     a: String,
//     b: String,
// }
// static mut config: Option<&mut Config> = None;
//
// /* 让代码工作，但不要修改函数的签名 */
// fn init() -> Option<&'static mut Config> {
//     Some(&mut Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     })
// }
//
//
// fn main() {
//     unsafe {
//         config = init();
//
//         println!("{:?}",config)
//     }
// }

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* 让代码工作，但不要修改函数的签名 */
fn init() -> Option<&'static mut Config> {
    // static mut c = Some(&mut Config {
    //     a: "A".to_string(),
    //     b: "B".to_string(),
    // });
    Some(Box::leak(Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    })))
}


fn main() {
    unsafe {
        config = init();

        println!("{:?}",config)
    }
}
