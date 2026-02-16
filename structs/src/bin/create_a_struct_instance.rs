use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum Genre {
    Action,
    OpenWorld,
    Adventure,
    Shooter,
    Unknown,
}

impl Genre {
    fn new(genre: &str) -> Self {
        match genre.trim().to_ascii_lowercase().as_str() {
            "action" => Genre::Action,
            "open-world" | "openworld" => Genre::OpenWorld,
            "adventure" | "adv" => Genre::Adventure,
            "shooter" => Genre::Shooter,
            _ => Genre::Unknown,
        }
    }

    fn to_string(&self) -> &str {
        return match self {
            Genre::Action => "Action",
            Genre::OpenWorld => "Open-World",
            Genre::Adventure => "Adventure",
            Genre::Shooter => "Shooter",
            Genre::Unknown => "Unknown",
        };
    }
}

struct Game {
    uuid: String,
    name: &'static str,
    year: i32,
    genre: Genre,
}

impl Game {
    fn new(uuid: String, name: &'static str, year: i32, genre: &str) -> Self {
        Game {
            uuid,
            name,
            year,
            genre: Genre::new(genre),
        }
    }
}

struct GameLibrary {
    username: &'static str,
    email: &'static str,
    games: Option<Vec<Game>>,
    game_count: i32,
}

impl GameLibrary {
    fn new(username: &'static str, email: &'static str) -> Self {
        GameLibrary {
            username,
            email,
            games: None,
            game_count: 0,
        }
    }

    fn add_game(&mut self, name: &'static str, year: i32, genre: &'static str) {
        let uuid: String = generate_uuid();
        let game: Game = Game::new(uuid, name, year, genre);

        // using get_or_insert here to add data to a
        // optional vector of game
        let games_vec: &mut Vec<Game> = self.games.get_or_insert(Vec::new());

        games_vec.push(game);
        self.game_count += 1;
    }
}

fn main() {
    let mut library = GameLibrary::new("PlayerOne", "player@example.com");

    library.add_game("Elden Ring", 2022, "open-world");
    library.add_game("Doom Eternal", 2020, "shooter");
    library.add_game("The Division2", 2019, "Open-World");

    let raw_data: Vec<(&str, i32, &str)> = vec![
        ("Elden Ring", 2022, "open-world"),
        ("Doom Eternal", 2020, "shooter"),
        ("The Witcher 3", 2015, "open-world"),
        ("Hades", 2020, "action"),
        ("Cyberpunk 2077", 2020, "open-world"),
        ("Sekiro: Shadows Die Twice", 2019, "action"),
        ("Apex Legends", 2019, "shooter"),
        ("God of War", 2018, "adventure"),
        ("Red Dead Redemption 2", 2018, "open-world"),
        ("Overwatch", 2016, "shooter"),
        ("Bloodborne", 2015, "action"),
        ("Skyrim", 2011, "open-world"),
        ("Halo Infinite", 2021, "shooter"),
        ("Uncharted 4", 2016, "adventure"),
        ("Titanfall 2", 2016, "shooter"),
        ("Ghost of Tsushima", 2020, "open-world"),
        ("The Last of Us Part II", 2020, "adventure"),
        ("Devil May Cry 5", 2019, "action"),
        ("Call of Duty: Warzone", 2020, "shooter"),
        ("Breath of the Wild", 2017, "open-world"),
    ];

    for game in raw_data {
        library.add_game(game.0, game.1, game.2);
    }

    println!("Library for: {} | {}", library.username, library.email);
    println!("Total games: {}", library.game_count);

    if let Some(games) = library.games {
        for game in games {
            println!(
                "[{}] {} ({}) - {:?}",
                game.uuid,
                game.name,
                game.year,
                game.genre.to_string()
            );
        }
    }
}

fn generate_uuid() -> String {
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    format!("{:x}", start)
}
