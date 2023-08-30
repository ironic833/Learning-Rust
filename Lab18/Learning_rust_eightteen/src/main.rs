struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32
}

struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 >= self.1 {self.0} else {self.1}
    }

    fn lesser(&self) -> i32 {
        if self.0 < self.1 {self.0} else {self.1}
    }
}


impl Person {

    fn new() -> Self {
        Self{
            name: String::from("name"),
            citizenship: String::from("from"),
            age: 1,
            gender: '?',
            salary: 10,
        }
    }

    fn computer_taxes(&self) -> f32{
        (self.salary as f32 / 3.) * 0.5
    }
}

fn main() {

    let person1 = Person{
        name: String::from("Oisin Hickey"),
        citizenship: String::from("Irish"),
        age: 20,
        gender: 'M',
        salary: 20_000,
    };

    let some_numbers: Numbers = Numbers(32,16);

    println!("This is the value returned {}", some_numbers.greater());

    println!("The structure properties are {} {} {}", person1.citizenship, person1.age, person1.gender);

    println!("The function outputs {}", person1.computer_taxes() );

    let person2 = Person::new();

    println!("The structure properties are {} {} {}", person2.citizenship, person2.age, person2.gender);

    println!("The function outputs {}", person2.computer_taxes() );

    let person3 = Person{
        name: String::from("Howya"),
        citizenship: String::from("Japanese"),
        .. person1
    };

    println!("The structure properties are {} {} {}", person3.citizenship, person3.age, person3.gender);

    let mut person4 = Person{
        name: String::from("Pers4"),
        citizenship: String::from("FromASpot"),
        .. person3
    };

    println!("The structure properties are {} {} {}", person4.citizenship, person4.age, person4.gender);

    person4.name = String::from("Asif");

    println!("The structure properties are {}", person4.name);

}
