fn main() {
    let s1 = String::from("hello");
    println!("{} is the slice of first two chars of {}", &s1[..2], s1);
}