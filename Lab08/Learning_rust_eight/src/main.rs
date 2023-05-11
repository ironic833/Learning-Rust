fn main() {
    let mut number_vec: Vec<i32> = vec![4,5,6,7,8,9,10,11,23,45,78];

    println!("{}", number_vec[0]);
    println!("{:?}", number_vec);

    number_vec[4] = 5;

    println!("{:?}", number_vec);

    let array_with_same_elements: Vec<i32> = vec![0;10];

    let mut string_array: Vec<&str> = vec!["apple", "tomato", "grapes"];

    let string_array_2: Vec<&str> = vec!["Unknown";6];

    string_array[0] = "kamran azam";

    let char_vec: Vec<char> = vec!['a','p','l','l','e'];

    let subset_vec: &&[i32] = &&number_vec[0..3];

    println!("The subset of values is {:?} ", subset_vec);

    println!("Elements in the vector are {:?} ", number_vec.len());

    let check_index: Option<&i32> = number_vec.get(100);

    println!("{:?} ", check_index);

    number_vec.push(30);
    number_vec.push(40);

    println!("Elements in the vector are {:?} ", number_vec);

    number_vec.remove(5);

    println!("Elements in the vector are {:?} ", number_vec);

    println!("The value of 10 exists {} ", number_vec.contains(&10));

}
