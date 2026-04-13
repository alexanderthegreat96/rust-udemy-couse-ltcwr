#![allow(dead_code)]
#![allow(unused_variables)]
enum Cheesesteak<T> {
    Plain,
    Topping(T),
}

#[derive(Debug)]
enum Vehicle<T, V> {
    Car,
    Suv(T),
    CylinderCapacity(V),
}

fn main() {
    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);

    let plain: Cheesesteak<String> = Cheesesteak::Plain;

    // Invalid, &str is not a String, which is what T must be for plain variable
    // plain = Cheesesteak::Topping("sausage");

    let suv: Vehicle<String, i32> = Vehicle::Suv(String::from("BMW X5 E53"));
    let cylinders: Vehicle<String, i32> = Vehicle::CylinderCapacity(12345);

    println!("{:?}", suv);
    println!("{:?}", cylinders);
}
