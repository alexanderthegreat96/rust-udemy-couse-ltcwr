fn main() {
    // the reason why we need a turbofish operator here
    // is because the parse method needs to know what we want to
    // extract the value as
    // this is ok because this string is an integer
    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    // this fails because this is not numeric data
    let text = "Alabama";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);
}

