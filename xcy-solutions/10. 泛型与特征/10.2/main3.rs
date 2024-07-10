// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
//
// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }
//
// // 修复 main 函数中的错误
// fn main() {
//     check_size([0u8; 767]);
//     check_size([0i32; 191]);
//     check_size(["hello你好"; __]); // size of &str ?
//     check_size([(); __].map(|_| "hello你好".to_string()));  // size of String?
//     check_size(['中'; __]); // size of char ?
// }
//
//
//
// pub enum Assert<const CHECK: bool> {}
//
// pub trait IsTrue {}
//
// impl IsTrue for Assert<true> {}

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// run shell: rustup default nightly

// 修复 main 函数中的错误
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 1]); // size of &str ?
    check_size([(); 1].map(|_| "hello你好".to_string()));  // size of String?
    check_size(['中'; 1]); // size of char ?
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
