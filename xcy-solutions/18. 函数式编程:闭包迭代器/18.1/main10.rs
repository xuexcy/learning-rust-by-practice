// /* Fill in the blank using two approaches,
//  and fix the error */
//  fn create_fn() -> __ {
//     let num = 5;
//
//     // How does the following closure capture the environment variable `num`
//     // &T, &mut T, T ?
//     |x| x + num
// }
//
//
// fn main() {
//     let fn_plain = create_fn();
//     fn_plain(1);
// }

/* Fill in the blank using two approaches,
 and fix the error */
fn create_fn() -> Box<dyn Fn(i32) -> i32>  {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    Box::new(move|x| x + num)
}
fn create_fn_v2() -> impl Fn(i32) -> i32  {
    let num = 5;

    // How does the following closure capture the environment variable `num`
    // &T, &mut T, T ?
    move|x| x + num
}

fn main() {
    let fn_plain = create_fn();
    fn_plain(1);
}
