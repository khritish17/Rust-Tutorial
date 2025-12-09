fn main() {
    // mutable variables
    let mut x = 10;
    println!("the value of x: {x}");
    x = 11;
    println!("The new value of x: {x}");

    // contstant
    const THREE_HOURSE_IN_SECONDS: i32 = 3 * 60 * 60;
    println!("Three hours in seconds: {THREE_HOURSE_IN_SECONDS}");

    // shadowing with scope
    let y = 15;
    println!("The orignal value of y is: {y}");

    let y = y + 1;
    println!("The value of y (after shadowing): {y}"); 
    
    {
        let y = y * 10;
        println!("The value of y inside the inner scope: {y}");
    }

    println!("The value of y outside the inner scope: {y}");

}
