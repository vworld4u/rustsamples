use std::io;

fn main() {
    println!("Enter the n (for finding nth Fibonacci number)");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Enter a number");
    let n: u32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => 5
    };
    println!("Finding {} th Fibonacci number", n);
    let mut f0: u64 = 1;
    let mut f1: u64 = 1;
    let mut f2: u64 = f0 + f1;
    let mut index = 1;
    while index < n {
        f0 = f1;
        f1 = f2;
        f2 = f0 + f1;
        index = index + 1;
    }
    println!("{} th Fibonacci number = {}", n, f2);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}