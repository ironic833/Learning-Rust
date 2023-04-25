fn main() {

    let first_number: i32 = 260;
    let second_number: f64 = 234.50;

    let large_number :i32= 1_000_000;

    let x :i32 = 255;

    println!("This is a set of numbers {:o}, {:x}, {:b}", x, x, x);

    let number :i32 = 45;

    let n1 :i32 = 14;
    let n2 :f32 = 64.8;

    let n3 = n1 + n2 as i32;

    println!("This variable is equals to {}", n3);

    // This is more accurate as it doesn't round
    let n4 = n1 as f32 + n2;

    println!("This variable is equals to {}", n4);

    let s :i32 = 5;
    let s :i32 = 5 * 5;

    println!("This variable is equals to {}", s);

    let r :i32 = 60;
    {
        let r :i32 = 65;
        println!("Inside the code segment: {}", r);
    }

    println!("Outside the code segment: {}", r);

    const MAX_SALARY:i32 = 100000;
}
