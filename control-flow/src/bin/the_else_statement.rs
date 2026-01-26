fn main() {
    let season = "spring";
    // else gets executed when if or elseif do not match
    // it's like a fallback statement
    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!");
    } else {
        println!("Lots of rain!");
    }
}
