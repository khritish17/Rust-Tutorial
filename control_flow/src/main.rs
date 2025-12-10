fn main() {
    // Control flow:
    // If else statement
    let number = 10;

    if number % 2 == 0{
        println!("{number} is even");
    } else {
        println!("{number} is odd");
    }

    // if number { // Gives an error cause if statement expects a bool but got integer
    //     println!("The number is {number}");
    // }

    let num = 100;
    // else if - condition
    if num % 2 == 0{
        println!("{num} is divisible by 2");
    } else if num % 3 == 0{
        println!("{num} is divisible by 3");
    } else if num % 5 == 0{
        println!("{num} is divisible by 5");
    }

    if num % 2 == 0{
        println!("{num} is divisible by 2");
    }
    if num % 3 == 0{
        println!("{num} is divisible by 3");
    }
    if num % 5 == 0{
        println!("{num} is divisible by 5");
    }

    // use if else as a teritary condition: if condition {} else {}
    let even: &str = if num % 2 == 0 {"even"} else {"odd"};
    println!("{even}");

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");


    // looping
    let mut i = 0;
    loop {
        i += 1;
        if i == 10{
            break
        }
    }
    println!("The value of i:{i}");

    // looping with return value
    let mut count = 0;
    let ans = loop {
        count += 1;
        if count == 10{
            break count * 2;
        }
    };
    println!("The value of ans: {ans}");

    // loop labels
    let mut i = 0;
    let mut j = 0;
    'outer_loop: loop {
        i += 1;
        'inner_loop: loop {
            j += 1;
            if j == 10{
                j = 0;
                break 'inner_loop
            }
            if i == 30{
                break 'outer_loop
            }
        }
    }
    println!("The outloop variable i : {i}");

    // looping with while
    let mut countdown = 10;

    while countdown > 0{
        countdown -= 1;
        println!("Rocket launch in T-minus {countdown} sec");
    }
    println!("Thats a liftoff folks");

    // for loop 
    for x in 1..5{
        println!("The value of x is {x}");
    }
    for rev_x in (1..5).rev(){
        println!("The value of rev_x is {rev_x}");
    }
    let a = [10, 20, 30, 40, 50];
    for arr_ele in a{
        println!("The array elements in a is {arr_ele}");
    }

}
