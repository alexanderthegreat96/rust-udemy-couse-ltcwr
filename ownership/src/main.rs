fn main() {
    // ownership is a compiler feature
    // responsible for allocating and deallocating memory
    // it works primarely with the heap as that's the spot
    // where dynamically sized values exist

    // stack -> faster, values are known at compile time
    // heap -> slower, values are dynamic, therefore unknown at compile time

    // owner:
    // variable
    // function parameter

    // stored in the heap
    let age: i32 = 10; // age is the owner of the value 10
    let smaller_age: i32 = age; // smaller age now owns age therfore, it ownes the value 10

    // variables live as long as a block lives
    // a block is {}, therefore, age will go out of scope once main finishes

    // stored in the both the heap and the stack
    // first in the stack
    let mut should_be_handsome: bool = false;
    {
        let is_handsome: bool = true;
        // this var only exists within this nested block
        // aka nested scope

        // now, heap
        if is_handsome {
            should_be_handsome = true;
        }
    }

    println!("Is he handsome?: {}", should_be_handsome);
}
