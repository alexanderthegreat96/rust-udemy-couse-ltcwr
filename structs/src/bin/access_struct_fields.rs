struct Player {
    username: String,
    level: u32,
    online: bool,
}

fn main() {
    let mut user1 = Player {
        username: String::from("Ace"),
        level: 10,
        online: true,
    };

    println!("User {} is at level {}", user1.username, user1.level);

    user1.level += 1;
    user1.online = false;

    println!("New level: {}", user1.level);
}
