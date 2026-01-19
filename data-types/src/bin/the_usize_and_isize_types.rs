fn main() {
    // the previous integer types use
    // a consistent amount of memory
    // ex: i32 -> 32 bits | i16 -> 16 bits
    // usize -> unsigned values
    // isize -> signed values
    // these 2 are aliases / nicknames for an existing type
    // but what these are depends on the os architecture
    // ex: usize = u32 on a 32 bit system or a u64 on a 64 bit computer
    // same thing for isize or signed integer
    // the advantage is that we can use 1 bit of code to work for different systems

    // in other words, the ocmpiler will automatically figure things out
    // depending on the CPU architecture
    let days: usize = 55;
    let years: isize = -15_000;
}
