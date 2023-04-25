fn main() {
    let some_string: &str = "Fixed length string";
    let mut growable_string: String = String::from("This string will grow");

    println!("The string is: \"{}\"", growable_string);

    growable_string.push('s');
    println!("The string is: \"{}\"", growable_string);
}
