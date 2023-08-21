use std::io::Stdin;

fn main() {
    
    println!("Enter a number!");

    let mut some_num = String::new();

    std::io::stdin().read_line( &mut some_num).expect("Failed to read input" );

    let some_num: i32  = some_num.trim().parse().expect("invalid input");

    if some_num != 0 {
        if some_num %2 == 0 {
            println!("The number is even");
        }
        else {
            println!("The number is odd");
        }
    } else {
        println!("Indivisible");
    }

    let value = if true {
        1
    } else {
        2
    };

    println!("Value = {}", value);

    let marks= 95;
    let mut grade: char = 'N';
    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else  {
        grade = 'F';
    }

    println!("The obtained grade is {}", grade);

    let marks= 65;
    let grade: char = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'D'
    } else  {
        'F'
    };

    println!("The obtained grade is {}", grade);



}
