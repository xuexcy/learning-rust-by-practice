//
// use std::ops::Sub;
//
// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// impl __ {
//     type Output = Self;
//
//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }
//
// fn main() {
//     assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 });
//
//     println!("Success!")
// }


use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// impl<T> Sub for Point<T>
// impl<T> Sub<Point<T>> for Point<T>
impl<T> Sub<Self> for Point<T>
where
    T: Sub<Output=T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 });

    println!("Success!")
}
