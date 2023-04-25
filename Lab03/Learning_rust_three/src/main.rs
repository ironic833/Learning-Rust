fn main() {
    let y: i32 = 15;
    println!("This is a int 32 {}", y);

    let x: f32 = 15.0;
    println!("This is a float 32 {}", x);

    let mut z: f32 = 17.0;
    println!("This is a float 32 {}", z);

    z = 18.0;
    println!("This is a float 32 {} which I've updated", z);

    let y: i32 = 5 * 5;

    println!("This maximum number in i8 is {}", std::i8::MAX);

    println!("This maximum number in u8 is {}", std::u8::MAX);

    let b: f64 = 3.65; 

    let status: bool = false;
    println!("The status is {:?}", (x,y,z, status));

    let not_equals: bool = 18 != 18;

    println!("The status is {}", not_equals);

    let c1: char = 'c';
    println!("The status is {}", c1);

}
