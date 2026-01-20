struct User {
    user_id: u64,
    user_name: String,
    lat: f64,
    long: f64,
}

impl User {
    pub fn new(user_id: u64, user_name: String, lat: f64, long: f64) -> User {
        return User {
            user_id: user_id,
            user_name: user_name,
            lat: lat,
            long: long,
        };
    }
    // if we don't use [&self]
    // the user instance is dropped
    // we can technically use [self] too
    pub fn print_user_details(&self) -> () {
        println!("User ID: {}", self.user_id);
        println!("User Name: {}", self.user_name);
        println!("Latitude: {}", self.lat);
        println!("Longitude: {}", self.long);
    }
}

fn main() {
    // rust enables conversion of types
    // using the "as" keyboard
    // which is really neat

    // "as" is mostly used for primitive data types
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8: u8 = miles_away as u8;

    let miles_away: f64 = 100.329032;
    let miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;
    println!("{miles_away_int}");

    let distance_to_city = miles_away as u64;
    println!("Distance to city: {}", distance_to_city);

    let user: User = User::new(1223, String::from("Alex"), 12.4434343, 13.345345);
    user.print_user_details();
}
