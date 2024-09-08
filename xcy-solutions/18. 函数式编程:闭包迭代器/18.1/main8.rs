// /* Fill in the blank */
// fn main() {
//     let mut s = String::new();
//
//     let update_string = |str| -> String {s.push_str(str); s };
//
//     exec(update_string);
// }
//
// fn exec<'a, F: __>(mut f: F) {
//     f("hello");
// }

/* Fill in the blank */
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    f("hello");
}
