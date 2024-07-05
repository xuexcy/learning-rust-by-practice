
// 修复错误
// enum Number {
//     Zero,
//     One,
//     Two,
// }
//
// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }
//
// // C语言风格的枚举定义
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }
//
//
// fn main() {
//     // 通过 `as` 可以将枚举值强转为整数类型
//     assert_eq!(Number::One, Number1::One);
//     assert_eq!(Number1::One, Number2::One);
// }


// 修复错误
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C语言风格的枚举定义
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn main() {
    // 通过 `as` 可以将枚举值强转为整数类型
    assert_eq!(Number::One as isize, Number1::One as isize);
    assert_eq!(Number1::One as isize, Number2::One as isize);
}
