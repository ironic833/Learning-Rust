use core::num;

fn main() {
    
    /* loop {
        println!("This is an infinte loop");
    } */

    let my_number = 5;
    let mut guess = false;

    println!("Guess my number which is between 1 and 20");

    while guess != true{
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).expect("failed to read input");

        let number: u8 = number.trim().parse().expect("invalid input");
        if my_number == number {
            println!("You guessed the number correctly");
            guess = true;
        } else {
            println!("Please try again");
        }
    }

    println!("Enter a number and I will tell you the next number after it that is divisible by both 2 and 5");

    let mut number = String::new();

    std::io::stdin().read_line(&mut number).expect("Failed to read input");

    let mut number: u8 = number.trim().parse().expect("invalid input");
    let mut divisible_by_2_5 = false;

    /* while divisible_by_2_5 != true {
        number = number + 1;
        if number % 2 == 0 && number % 5 == 0 {
            println!("The number after your number divisible by both 2 and 5 is {}", number);
            divisible_by_2_5 = true;
        }
    } */

    number = number + 1;

    while (number % 2 == 0 && number % 5 == 0)  != true {
        number = number + 1;
    }

    println!("The number after your number divisible by both 2 and 5 is {}", number);  


}
