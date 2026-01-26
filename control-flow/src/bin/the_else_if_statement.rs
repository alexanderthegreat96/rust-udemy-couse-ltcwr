fn main() {
    // quite easy
    // only applied after an initial if statement
    // checks for another condition after if

    let season = "summer";

    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!");
    } else if season == "fall" {
        println!("Leaves falling!")
    } else if season == "spring" {
        println!("Lots of rain!")
    }

    if season == "summer" {}

    if season == "winter" {}
}
