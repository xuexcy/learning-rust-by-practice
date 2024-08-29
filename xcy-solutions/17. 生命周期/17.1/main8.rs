//
// #[derive(Debug)]
// struct NoCopyType {}
//
// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }
//
// /* 修复函数的签名 */
// fn fix_me(foo: &Example) -> &NoCopyType
// { foo.b }
//
// fn main()
// {
//     let no_copy = NoCopyType {};
//     let example = Example { a: &1, b: &no_copy };
//     fix_me(&example);
//     println!("Success!")
// }


#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

/* 修复函数的签名 */
fn fix_me<'a: 'b, 'b>(foo: &'a Example) -> &'b NoCopyType
{ foo.b }
/* 修复函数的签名 */
fn fix_me2<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType
{ foo.b }

fn main()
{
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}
