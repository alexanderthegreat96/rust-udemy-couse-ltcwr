// intro to methods
// methods are functions that are defined on a type
// they are called using dot notation

// it's a function that lives on a value
// it lives on a type
// value.method_name()

// method attatched to a struct / custom type
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn greet(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

// method attatched to a primitive type
// using traits aka interfaces

trait NumberMethods {
    fn abs(&self) -> i32;
    fn pow(&self, exp: u32) -> i32;
}

impl NumberMethods for i32 {
    fn abs(&self) -> i32 {
        if *self < 0 { -*self } else { *self }
    }

    fn pow(&self, exp: u32) -> i32 {
        let mut result = 1;
        for _ in 0..exp {
            result *= *self;
        }
        result
    }
}

trait StringMethods {
    fn trim(&self) -> String;
    fn to_chars(&self) -> Vec<char>;
}

impl StringMethods for Person {
    fn trim(&self) -> String {
        self.name.trim().to_string()
    }

    fn to_chars(&self) -> Vec<char> {
        self.name.chars().collect()
    }
}

fn main() {
    let val: i32 = -15;
    println!("{}", val.abs());

    let empty_space = "     my content    ";
    println!("{}", empty_space.trim());

    println!("{}", val.pow(2));
    println!("{}", val.pow(3));

    // person

    let person: Person = Person::new(String::from("Mike  "), 50);
    person.trim();
    person.greet();

    println!("{:?}", person.to_chars());
}
