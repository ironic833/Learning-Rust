struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

trait generalInformation {
    fn info (&self) -> (&str, u8, char);

    fn country_info (&self) -> &str;
}

trait GeneralInfo {
    fn area (&self) {
        println!("i am not implemented for this type");
    }

    fn perimeter (&self);
}

impl GeneralInfo for Circle {
    fn area (&self) {
        let area_of_a_circle: f32 = 3.14 * (self.radius * self.radius);
        println!("The area of the circle is {:?}", area_of_a_circle);
    }

    fn perimeter (&self) {
        let circumference = 2.0 * 3.14 * self.radius;
        println!("The circumference is {:?}", circumference);
    }
}

impl GeneralInfo for Rectangle {
    fn area (&self) {
        let area_of_a_rectangle = self.length * self.width;

        println!("The area is {:?}", area_of_a_rectangle);
    }

    fn perimeter (&self) {
        let perimeter_of_a_rectangle = 2.0 * (self.length * self.width);
        println!("The area of the rectangle is {:?}", perimeter_of_a_rectangle);
    }
}


impl generalInformation for Person {
    fn info (&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)
    }

    fn country_info (&self) -> &str {
        &self.citizenship
    }
}

impl generalInformation for Student {
    fn info (&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }

    fn country_info (&self) -> &str {
        &self.country
    }
}


fn main() {

    let c1: Circle = Circle { radius: 3.2, };

    let r1 = Rectangle{ width: 5.0, length: 6.0,};
    
    let person1: Person = Person { citizenship: String::from("Irish"), name: String::from("Oisin"), age: (22), gender: ('M'), salary: (20_000) };
    let student1: Student = Student { name_std: String::from("Squish"), age: (21), sex: ('F'), country: String::from("Malaysia") };

    println!("The basic info include {:?}", person1.info());
    println!("The basic info for the student is {:?}", student1.info());

}
