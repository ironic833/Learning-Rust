

fn main() {

    let some_number = 40;

    if some_number < 50 {
        println!("The number is less than 50");
    }

    println!("The line will exclude irrespective of the condition of the if statement");

    let marks = 65;

    if marks >= 60 && marks <= 70 {
        println!("The grade is satisfactory");
    }

    let flag_1 = true;
    let flag_2 = false;

    if flag_1 == true || flag_2 == true {
        println!("One of the conditions is true here");
    }
    
    let flag_1 = false;

    if flag_1 != true {
        println!("Will execute when flag is not true");
    }

    let flag_1 = true;
    let flag_2 = false;

    let number = 60;

    if (flag_1 == true && flag_2 == false) || number < 50 {
        println!("Testing the conditions");
    }

    let marks = 80; 

    if marks > 50 {
        println!("You have passed");
    } else {
        println!("You have failed");
    }

    let marks = 95; 
    let mut grade: char = 'N';


    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }

    println!("The obtained grade is {}", grade);

}
