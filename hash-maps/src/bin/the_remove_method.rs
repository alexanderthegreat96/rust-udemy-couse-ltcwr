use std::collections::HashMap;

fn main() {
    // an array of 3 tuple elements
    let data: [(&str, i32); 3] = [("Bobby", 7), ("Grant", 4), ("Ben", 6)];

    // from simply takes an array of tuple values
    // so this way you can prepopulate debug_assert_eq
    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);

    // remove deletes the key value pair
    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);
    println!("{}", ben.unwrap());
    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben);
    println!("{:?}", years_at_company);

    let games_list: [(&str, i32); 4] = [
        ("The Division2", 2019),
        ("Cyberpunk 2077", 2020),
        ("Ghost Recon Breakpoint", 2020),
        ("Metin2", 2006),
    ];

    let mut games: HashMap<&str, i32> = HashMap::from(games_list);
    println!("Games: {:#?}", games);
    // remove returns an option of i32
    // if let Some(removed) = games.remove("Metin2") {
    //     println!("Just removed: {}", removed);
    // } else {
    //     println!("Unable to wipe the value");
    // }
    if games.remove("Metin2").is_none() {
        println!("Unable to find value to remove");
    }
    println!("Games: {:#?}", games);
}
