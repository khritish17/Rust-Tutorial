use std::io;

fn main() {
     
    // Here parsing "42" will result in any of this: i32, u32, i64, u64 etc. 
    // hence the rustc needs to know what data type its, as rust is staticaly types language 
    // let x: i32 = "42".parse().expect("Not a number");
    let x: i32 = "420".parse().expect("Not a number");
    println!("The value of x: {x}");

    // Integer data type
    let signed_int = -10; // it can be i8, i16, i32, i64, i128, isize (based on the working computer arcitecture)
    let unsigned_int: u32 = 100; // it can be u8, u16, u32, u64, u128, usize (based on the working computer architecture)

    // Floating point data type:
    let pi = 3.14;

    // Boolean type
    let is_good = true;

    // character type
    let vowel = 'a';
    let heart_eyed_cat = '😻';

    // compound type 
    // tuple
    let tup = (1, 2.0, true);
    let first_val = tup.0;
    println!("{first_val}");

    // array and accessing an array

    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to readline");

    let index: usize = index.trim().parse().expect("Not a number");

    let ele = a[index];
    println!("The value of a[{index}] = {ele}");

}
