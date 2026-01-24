// the block is like a mini function
// it allows for early memory cleanup
// and isolated nested execution
fn another_function() {
    let total_value: i32 = 0;

    let compute: i32 = {
        let weight: f64 = 1.2;
        let total = total_value + 10;

        // convert 'total' to f64 to match 'weight',
        // then convert the final result back to i32
        (total as f64 / weight) as i32
    };

    println!("Computed value: {}", compute);
}

fn main() {
    let multiplier = 3;

    // blocks
    // as a reminder
    // when using a block, you will isolate the code in the
    // curly brackets from the rest of the code
    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };

    println!("{calculation}");
    another_function();
}
