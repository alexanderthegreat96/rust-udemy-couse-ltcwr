fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    // positional args here are used using
    // indexes from 0 to n
    println!(
        "This year, my garden has {0} apples and {1} oranges. I can't believe I have {0} apples!",
        apples, oranges
    );
}
