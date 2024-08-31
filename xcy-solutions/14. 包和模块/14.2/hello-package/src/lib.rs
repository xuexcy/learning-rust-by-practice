pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
        fn complain() {}
    }
}

fn eat_at_restaurant() {
    // 使用绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 使用相对路径调用
    front_of_house::hosting::add_to_waitlist();
}

// in lib.rs

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 使用2种方式填空
        //1. 使用关键字 `super`
        //2. 使用绝对路径
        crate::front_of_house::serving::serve_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}
