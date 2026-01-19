// in rust variables are immutable by default
// if you want to make them mutable, you gotta provide the mut keyword

fn do_reps(reps: &mut i32, rep: i32) {
    *reps += rep;
}

fn main() {
    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    println!("I now plan to do {gym_reps} reps");

    do_reps(&mut gym_reps, 2);
    do_reps(&mut gym_reps, 6);
    do_reps(&mut gym_reps, 8);

    println!("I just finished doing: {} reps.", gym_reps);
}

// in rust
// we can use rustc --explain ERROR_CODE
// example: rustc --explain E0425
// we can also search for rust error codes index
