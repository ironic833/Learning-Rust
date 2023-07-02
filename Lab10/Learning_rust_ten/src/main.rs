
fn stack_function(mut var:i32) {
    var = 56;
    println!("Var: {}", var);
}

fn heap_function(var: &mut Vec<i32>){
    var.push(50);
    println!("Var: {:?}", var);
}

fn main() {

    let stack_num: i32 = 32;
    let mut heap_vec: Vec<i32> = vec![4,5,6];

    stack_function(stack_num);
    println!("The value inside the main of stack_num: {}", stack_num);

    heap_function(&mut heap_vec);
    println!("The value inside the main of heap_vec: {:?}", heap_vec);

    let some_vec: Vec<i32> = vec![4,5,6];

    let ref1: Vec<i32> = some_vec;
    let ref2 = &ref1;

    /* let mut vec_1: Vec<i32> = vec![4,5,6];
    let mut ref1 = &vec_1; */

    let large_data1: String = String::from("This is the first long string");
    let large_data2: String = String::from("This is the second long string");

    let huge_data: Vec<&String> = vec![&large_data1,&large_data2];

}
