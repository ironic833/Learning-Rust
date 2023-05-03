fn main() {
    let myInfo: (&str, i32) = ("Salary", 40_000);

    println!("{} is equal to {}", myInfo.0, myInfo.1);

    println!("Tuple is {:?}", myInfo);


    let (Salary, Salary_value) = myInfo;

    let Salary: &str = myInfo.0;
    let Salary_amount: i32 = myInfo.1;

    let someInfo: (&str, i32, (&str, i32)) = ("Salary", 40_000, ("car", 200));
}
