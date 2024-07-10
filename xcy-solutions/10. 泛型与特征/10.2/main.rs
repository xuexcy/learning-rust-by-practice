// struct ArrayPair<T, const N: usize> {
//     left: [T; N],
//     right: [T; N],
// }
//
// impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
//     // ...
// }

use std::fmt::Debug;
use std::fmt::Formatter;
use std::slice;

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    // ...
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!(
            "left: [{}]",
            self.left
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!(
            "right: [{}]",
            self.right
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(())
    }
}

pub struct MinSlice<T, const N: usize> {
    pub head: [T; N],
    pub tail: [T],
}
impl<T: std::fmt::Debug, const N: usize> MinSlice<T, N> {
    pub fn from_slice(slice: &[T]) -> Option<&MinSlice<T, N>> {
        if slice.len() >= N {
            Some(unsafe {Self::from_slice_unchecked(slice)})
        } else {
            None
        }
    }
    unsafe fn from_slice_unchecked(slice: &[T]) -> &MinSlice<T, N> {
        // 在 slice 原地址进行引用
        println!("slice len: {}, slice: {:?}", slice.len(), slice);
        let resized_len = slice.len() - N;
        // pub const unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T]
        let resized = slice::from_raw_parts(slice.as_ptr(), resized_len);
        println!("resized len: {}, resized: {:?}", resized_len, resized);
        println!("slice addr {:?}", (slice as *const [T]));
        println!("resized addr {:?}", (resized as *const [T]));
        //&*(resized as *const [T] as *const MinSlice<T, N>)
        &*(resized as *const [T] as *const MinSlice<T, N>)
    }
}

fn main() {
    let ap1 = ArrayPair { left: [1, 2, 3, 4, 5], right: [6, 7, 8, 9, 10]};
    println!("{:?}", ap1);

    let slice: &[u8] = b"hello, world";
    // 编译器不知道slice长度
    let reference: Option<&u8> = slice.get(6);
    assert!(reference.is_some());
    // 优化;
    let slice: &[u8] = b"hello, worldtail";
    let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    let value: u8 = minslice.head[6];
    assert_eq!(value, b' ');
    println!("{}", minslice.tail.len());
    println!("{:?}", (slice as *const [u8]));
    println!("{:?}", (&minslice.head as *const [u8]));
    println!("{:?}", (&minslice.tail as *const [u8]));
    println!("{}",
        minslice.tail
            .iter()
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>().join(", "));

}
