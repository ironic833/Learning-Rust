fn main() {
    let mut heap_num: Vec<i32> = vec![4,5,6];
    let ref1 = &mut heap_num;
    let ref2 = &mut heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let mut heap_num: Vec<i32> = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let mut heap_num: Vec<i32> = vec![4,5,6];
    let ref1: &Vec<i32> = &heap_num;
    let ref2: &Vec<i32> = &heap_num;
    let ref3: &Vec<i32> = &mut heap_num;
    println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref1, ref2, ref3);

    let mut heap_num: Vec<i32> = vec![4,5,6];
    let ref1: &Vec<i32> = &heap_num;
    let ref2: &Vec<i32> = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    let ref3: &Vec<i32> = &mut heap_num;

    let mut heap_num: Vec<i32> = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;

    heap_num.push(68);
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

}
