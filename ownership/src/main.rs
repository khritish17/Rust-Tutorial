fn main() {
    let x = 5;
    let y = x;

    println!("value of x: {x}");
    println!("value of y: {y}");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2} world");
    println!("The ownership of s1 is given to s2. visit 'https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#:~:text=s1%20and%20bind%20it%20to%20s2'");

}
