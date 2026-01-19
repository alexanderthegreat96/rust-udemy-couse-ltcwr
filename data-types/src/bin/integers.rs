fn main() {
    // uses less memory
    // but we also have less values we can store
    // -128, 128
    let eight_bit: i8 = 112;
    let eight_bit_unsigned: u8 = 112; // a negative value does not work

    // 16 bits or 8 bytes
    // ranges from -32768, 32767
    let sixteen_bit: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64000;

    // uses 32 bits or 16 bytes
    let thirty_two_bit_signed: i32 = 1200000; // allows negatives
    let thirty_two_bit_unsigned: u32 = 2500000; // does not allow negatives

    // we can also declare the type a bit differently

    // i did value and next to it, its type
    // ex: 10i8
    let test_type_unsigned_eight = 10i8;
}
