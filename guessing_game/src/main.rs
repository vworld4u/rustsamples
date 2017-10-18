extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guessing Game!");
    let sec_num = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number generated is {}", sec_num);
    loop {
        println!("Enter your guess below");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Congrats! You win!");
                break;
            }
        }
    }
}
