fn main() {
    basic();

    function_with_inputs("hickey", 40000);

    let full_name: &str = "Hickey";
    let salary_info: i32 = 50_000;

    function_with_inputs(full_name, salary_info);

    let answer: i32 = function_with_inputs_and_outputs( 10, 15);

    println!("The result is {}", answer);
    
    let (multiplication, addition, subtraction) = function_with_inputs_and_multiple_outputs(10, 20);

    println!("Multiplication: {}, addition: {}, division: {}", multiplication, addition, subtraction);

    let result: (i32, i32, i32) = function_with_inputs_and_multiple_outputs(10, 40);

    println!("Multiplication: {}, addition: {}, division: {}", result.0, result.1, result.2);

    let full_name: String = {
        let first_name = "Oisin";
        let last_name = "Hickey";

        format!("{} {}", first_name, last_name)
    };

    println!("My full name is {}", full_name);

    let mut n = String::new();

    std::io::stdin().read_line(&mut n).expect("failed to read input.");

    let n: f64 = n.trim().parse().expect("Invalid Input");

    println!("Input: {:?}" , n);
}

fn basic(){
    println!("This is a basic function");
}

fn function_with_inputs(name: &str, salary: i32){
    println!("The name is {} and the salary is {}", name, salary);
}

fn function_with_inputs_and_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_with_inputs_and_multiple_outputs(num1: i32, num2: i32) -> (i32, i32, i32 ) {
    (num1 * num2, num1 + num2, num1 - num2)
}
