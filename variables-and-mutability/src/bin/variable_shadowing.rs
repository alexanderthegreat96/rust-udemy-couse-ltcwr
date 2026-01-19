fn main() {
    // variable shoadowing
    let grams_of_protein = "100.345";
    // line 5 invalidates number3
    let grams_of_protein = 100.345;
    // same concept here, line 5 is replaced by 7
    let mut grams_of_protein = 100;
    // this does not apply here
    // since we are mutating the value
    // not redeclare it
    grams_of_protein = 105;

    // in other words
    // the old variable is replaced by the new one
}
