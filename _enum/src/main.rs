fn main() {
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move{x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    struct Quit;

    impl Message {
        fn call(&self){
            println!("h")
        }
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let some_number = Some(1);
    // None必须声明类型，因为编译器无法自动推导
    let none_number:Option<i32> = None;
    println!("some_number: {}", some_number.unwrap());
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => (),
    }
    let some_value = Some(0);
    if let Some(0) = some_value {
        println!("bingo")
    } else {
        println!("not match")
    }
}
