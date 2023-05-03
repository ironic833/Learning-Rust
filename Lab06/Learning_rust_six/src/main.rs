fn main() {
    let some_string: &str = "Fixed length string";
    let mut growable_string: String = String::from("This string will grow");

    println!("The string is: \"{}\"", growable_string);

    growable_string.push('s');
    println!("The string is: \"{}\"", growable_string);

    growable_string.pop();
    println!("The string is: \"{}\"", growable_string);

    growable_string.push_str(" which I will use");
    println!("The string is: \"{}\"", growable_string);

    println!(
        "Basic functions on strings,
        is empty(): {},
        length: {},
        Bytes(): {},
        Contains use: {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    growable_string.push_str("         ");
    let str_len: usize = growable_string.trim().len();

    let number: i32 = 6;
    let num_str: String = number.to_string();

    println!("Is the number a string: {}", num_str == "6");

    let some_char: char = 'a';
    let char_str: String = some_char.to_string();

    let my_name: String = "oisin".to_string();

    let empty_string: String =  String::new();

    println!("Length is {}", empty_string.len());

    let s_1: String = "Oisin".to_string();
    let s_2: String = "Hickey".to_string();
    let s_3: String = format!("My name is {} {}", s_1, s_2);
    println!("{}", s_3);

    let concat: String = format!("{}{}", s_1, s_2);
}
