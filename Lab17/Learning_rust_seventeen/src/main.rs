fn main() {
    
    /* let mut var = 100;

    loop{
        var = var -1;

        if var % 13 == 0 {
            break;
        }
    }

    println!("The highest number lesser then the given number divisible by 13 is {}", var); */

    let mut var = 0;

    let mut count = 0;

    let req_number: i32 = loop{

        var = var + 1;

        if var % 5 == 0 && var % 3 == 0 {

            println!("The number which is divisible by both three and five is {} \n", var);
            count = count + 1;

            if count == 3 {
                break var;
            } else {
                continue;
            }

        }

        println!("{}", var);

    };


    println!("The rquired third highest number is {}", req_number);

}
