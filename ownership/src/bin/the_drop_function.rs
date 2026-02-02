fn main() {
    let person = String::from("Boris");
    // this is normally called automatically
    // when the ownership is shifted
    drop(person);

    // let genius = person; // this fails and it is invalid
}
