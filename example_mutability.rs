fn main() {
    // Example 1: Variable mutability (mut on the variable)
    let mut x = 5;
    x = 10;  // OK - we can change what x holds because it's mut
    
    // Example 2: Reference mutability (mut on the reference)
    let mut my_string = String::from("Hello");
    
    // Without &mut, the function can't modify the string
    // add_text(&my_string);  // ERROR - can't modify through immutable reference
    
    // With &mut, the function CAN modify the string
    add_text(&mut my_string);  // OK - mutable reference allows modification
    println!("After modification: {}", my_string);
}

fn add_text(s: &mut String) {
    s.push_str(" World");  // This modifies the string's contents
}
