pub fn editor() {
    let mut my_str = String::from("Rust");

    // TODO: Call your function here
    add_and_measure(&mut my_str);
    
    // TODO: Print the results
    println!("{my_str}");
}

// TODO: Write the add_and_measure function
fn add_and_measure(str: &mut String) {
    str.push_str("-modified");
}