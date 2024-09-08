// fn main() {
//     {
//         // 字符串字面量能跟程序活得一样久，因此 `static_string` 的生命周期是 `'static`
//         let static_string = "I'm in read-only memory";
//         println!("static_string: {}", static_string);
//
//         // 当 `static_string` 超出作用域时，该引用就无法再被使用，但是引用指向的数据( 字符串字面量 ) 依然保存在二进制 binary 所占用的内存中
//     }
//
//     println!("static_string reference remains alive: {}", static_string);
// }
//

fn main() {
    let mut p: usize;
    let mut len: usize;
    {
        // 字符串字面量能跟程序活得一样久，因此 `static_string` 的生命周期是 `'static`
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
        p = static_string.as_ptr() as usize;
        len = static_string.len();
        // 当 `static_string` 超出作用域时，该引用就无法再被使用，但是引用指向的数据( 字符串字面量 ) 依然保存在二进制 binary 所占用的内存中
    }

    use std::slice::from_raw_parts;
    use std::str::from_utf8_unchecked;
    fn get_str_at_location(p: usize, len: usize) -> &'static str {
        unsafe { from_utf8_unchecked((from_raw_parts(p as *const u8, len))) }
    }
    println!("static_string reference remains alive: {}", get_str_at_location(p, len));
}
