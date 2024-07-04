// 用两种方法求解
fn main() {
    never_return();
    // never_return2();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    panic!("go panic without return");
}

fn never_return2() -> ! {
    // 实现这个函数，不要修改函数签名!
    loop {}
}
fn never_return3() -> ! {
    // 实现这个函数，不要修改函数签名!
    todo!();
}
