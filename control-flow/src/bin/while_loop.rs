fn main() {
    let mut seconds = 21;

    // run this look based on a condition
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }

    // infinite loop
    let mut number: i32 = 10;
    // the compiler complains
    // and will tell you to use loop
    while true {
        if number == 0 {
            println!("Hit 10");
            break;
        }

        number += 1;
    }
    println!("Blastoff! 🚀");
}
