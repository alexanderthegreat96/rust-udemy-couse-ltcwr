// Hours, minutes
struct ShortDuration(i32, i32);
// Years, months
struct LongDuration(i32, i32);

// some other example
// this is used for destructuring
struct Names(&'static str, &'static str);

fn main() {
    // a tuple struct is a struct that assigns multiple pieces
    // of data an order in line rather than a name
    // this is just a fancy / custom tuple type
    let work_shift = ShortDuration(8, 30);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    // go_to_work(era);
    // accept_tuple(era);

    // let work_shift = (8, 0);
    // let era = (5, 3);
    // go_to_work(work_shift);
    // go_to_work(era);

    let names: Names = Names("Alex", "Nobody");
    come_to_work(names);
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}

// we can destructure into 2 variables
fn come_to_work(Names(first_name, last_name): Names) {
    println!("{first_name} {last_name} is coming to work");
}

fn accept_tuple(length: (u32, u32)) {}
