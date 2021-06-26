fn main() {
   println!("hello world");
   let x = another_function();
   let y = five();
   println!("x: {}, y: {}", x, y);
   let z = 5;
   if z < 6 { 
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let condition = true;
    let number = if condition {
        8
    }else {
        9
    };
    println!("the value of number is: {}", number);
    let mut counter = 0;
    let result = loop {
       
        counter += 1;
       
        if counter == 10 {
            break counter*2;
        };
    };
    let mut count = 3;
    while count != 0 {

        println!("{}!", count);
        count -= 1;
    }
    
    println!("result is {}", result);

    let a = [10, 20, 30, 40];
    for element in a.iter() {
        println!("this value is: {}", element)
    }
    for num in (1..4).rev() {
        println!("num is: {}", num);
    }
    let fb = f(2);
    println!("fb: {}", fb);
   
}
fn five() -> u32 {
    5
}
fn another_function() -> u32{
    let _s = 5;
    let y = {
        let _s = 6;
        _s + 10
    };
    y
}

fn f (num: u32) -> u32 {
    if num <= 2 {
        return 1;
    }
    (num) + f(num - 1)
}