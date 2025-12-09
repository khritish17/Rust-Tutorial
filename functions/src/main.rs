fn main() {
    println!("Hello, world!");
    second(10);
    third('a', 8);
    let mul = function_with_return(2, 4);
    println!("The nul value: {mul}")
}

fn second(x:i32){
    println!("The value passed is :{x}");
}

fn third(label:char, x:i32){
    println!("The label:{label} and the value: {x}");
}

fn function_with_return(a:i32, b:i32)-> i32{
    let multiply = a * b;
    multiply // works as the return statement without return keyword and without the trailing semi-colon
    // return multiply;
}