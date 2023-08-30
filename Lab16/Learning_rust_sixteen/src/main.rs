fn main() {
    let mut some_vec = vec![45,30,85,90,41,39];

    for i in 0..=5 {
        println!("The {}th value in the vector is {}", i, some_vec[i]);
    }

    for i in some_vec.iter_mut() {
        *i += 5;
        println!("{}",i);
    }

    println!("{:?}", some_vec);


}
