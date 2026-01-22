fn main() {
    // like an array
    // is a collection type that can contain
    // multiple elements, each of which contain values
    // thse values can or not be homogenous
    // a tuple can store elements of different types
    let mut employee = ("Molly", 32, "Marketing");

    // tuples are also 0 indexed
    // which means you can access them using numeric keys
    // what's important here is that from arrays
    // which can be accessed like arr[0], tup.0
    // so you use .{index_value}

    // unlike python, tuples can be mutable and not only immutable

    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    let (name, age, department) = employee; // inline assignment
    println!("Name: {name}, age: {age}, department: {department}");

    // modified tuple values
    employee.0 = "Anna";
    employee.1 = 25;
    employee.2 = "Software";

    println!(
        "Name: {}, age: {}, department: {}",
        employee.0, employee.1, employee.2
    );
}
