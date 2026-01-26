fn main() {
    // iterations
    // doing something over and over again
    // using it to automate manual processes
    let mut seconds = 10;

    // this runs forever
    // for needs a start and end
    // loop does not
    loop {
        // runs backwards
        // because we are starting with 10 seconds
        // and subtracting 1 with each iteration

        if seconds == 0 {
            println!("Blastoff! 🚀");
            break;
        }
        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
}
