fn main() {
    // range is a sequence / interval of consecutive values
    let month_days = 1..31;
    println!("{month_days:?}");

    let month_days = 1..=31;
    println!("{month_days:?}");

    for number in month_days {
        print!("{number}");
    }

    // create a range of characters
    // range is an example of an iterable type
    let letters = 'a'..'f';

    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Yellow"];

    for color in colors {
        println!("{color} is a great color!");
    }

    // goes up to 100
    // up to 100 but not including 101
    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..101 {
        numbers.push(i);
    }

    println!("Numbers to 100: {:?}", numbers);

    // there is another syntax we can use to reach 100 without incrementing to 101
    // ..=100
    numbers.clear(); // clear the vector

    for i in 0..=100 {
        numbers.push(i);
    }

    println!("Numbers to 100: {:?}", numbers);

    let mut letters: Vec<char> = Vec::new();
    for char in 'a'..'z' {
        letters.push(char);
    }

    // since a string is a collection of chars
    // we can simply chain .iter().collect();
    // and this returns us the alphabet stringified
    let alphabet: String = letters.iter().collect();
    println!("The alphabet has thse chars: {}", alphabet);
}
