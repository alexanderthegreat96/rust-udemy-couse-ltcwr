fn main() {
    // basically a resizeable array
    // an array is fixed size
    // a more flexible array
    // it's a container that stores values of the same type in order
    
    let food_array: [i32; 6] = [1,2,3,4,5,6];
    println!("My numbers: {:?}", food_array);

    let pizza_diameters: Vec<i32> = vec![8, 10, 12, 14];
    println!("{pizza_diameters:?}");

    let pastas: Vec<&str> = vec!["Rigatoni", "Angel hair", "Fettucine"];
    println!("{pastas:?}");

    let mut waters = Vec::<String>::new();
    waters.insert(0, "Borsec".to_string());
    println!("We got waters: {:?}", waters);
}
