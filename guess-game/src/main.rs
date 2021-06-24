use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(1,10);
        println!("secret_number is {}", secret_number);
        let mut message = String::new();
        io::stdin().read_line(&mut message).expect("Failed to read line");
        let message: u32 = match message.trim().parse(){
            Ok(num) => num,
            Err(e) => continue,
        };
        match message.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
