fn main() {
    // boolean data type
    println!("!true is: {}", !true);
    println!("!false is: {}", !false);

    let age: i32 = 31;
    let is_young: bool = age < 31;
    let is_old: bool = !is_young;

    println!("{} | {} | {}", age, is_young, is_old);
}
