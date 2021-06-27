fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in: u64,
        active: bool,
    }
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {width:30,height:50};
    let rect2 = Rectangle {width: 10, height:20};
    let rect3 = Rectangle {width: 50, height:60};

    println!("the area of the rectangle is {}", rect1.area());
    println!("can rect1 hold rect2:  {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3:  {}", rect1.can_hold(&rect3));



    let mut user = User {
        username: String::from("lsf"),
        email: String::from("shmilyvidian@163.com"),
        sign_in: 1,
        active: true,
    };
    user.username = String::from("s");
    println!("username is: {:#?}", user);

}
