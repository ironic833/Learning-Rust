fn main() {
    let myInfo: (&str, i32) = ("Salary", 40_000);

    println!("{} is equal to {}", myInfo.0, myInfo.1);

    println!("Tuple is {:?}", myInfo);


    let (Salary, Salary_value) = myInfo;

    let Salary: &str = myInfo.0;
    let Salary_amount: i32 = myInfo.1;

    let someInfo: (&str, i32, (&str, i32)) = ("Salary", 40_000, ("car", 200));
    let element = someInfo.2.0;

    let empty_tuple: () = {};

    let mut number_array: [i32;5] = [4,5,6,8,9];

    println!("{}", number_array[0]);

    println!("{:?}", number_array);

    number_array[4] = 5;

    let array_with_same_elements: [i32; 10] = [0; 10];

    let mut string_array_1: [&str; 3] = ["apple", "tomato", "grapes"];

    let string_array_2 = ["unknown"; 6];

    string_array_1[0] = "karam azam";

    let char_array: [char; 5] = ['a','p','p','l','e'];

    let mut number_array_1 = [4,5,6,8,9];

    let subset_array = &number_array_1[0..3];

    print!("The subset of values is {:?}", subset_array);

    print!("Length of array is {} ", number_array_1.len());

    print!("Size of array is {} bytes ", std::mem::size_of_val(&number_array_1));

}
