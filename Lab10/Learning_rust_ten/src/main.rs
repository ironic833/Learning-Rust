
fn stack_function(mut var:i32) {
    var = 56;
    println!("Var: {}", var);
}



fn main() {

    let stack_num: i32 = 32;
    let mut heap_vec: Vec<i32> = vec![4,5,6];

    stack_function(stack_num);
    println!("The value inside the main of stack_num: {}", stack_num);

}
