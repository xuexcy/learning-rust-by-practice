
fn main() {
    {
        // 使用尽可能多的方法来通过编译
        let x = String::from("hello, world");
        let y = x.clone();
        println!("{},{}",x,y);
    }
    {
        // 使用尽可能多的方法来通过编译
        let x = "hello, world";
        let y = x;
        println!("{},{}",x,y);
    }
    {
        // 使用尽可能多的方法来通过编译
        let x = String::from("hello, world");
        println!("{}",x);
    }
    {
        // 使用尽可能多的方法来通过编译
        let x = &String::from("hello, world");
        let y = x;
        println!("{},{}",x,y);
    }
    {
        // 使用尽可能多的方法来通过编译
        let x = String::from("hello, world");
        let y = x.as_str();
        println!("{},{}",x,y);
    }
}

