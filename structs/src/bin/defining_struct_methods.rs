use std::fmt::Error;

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// methods are basically functions that belong to a type
// they are defined using the impl keyword -> implementation
// think of them as the way rust implement it's own OOP
impl TaylorSwiftSong {
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }
}

fn main() {
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.display_song_info();

    let connection: Connection =
        Connection::new(HttpScheme::HTTP, "localhost.com", 34, "mike", "mike");

    connection.connect();
}

#[derive(Debug)]
enum HttpScheme {
    HTTPS,
    HTTP,
}

struct Connection {
    scheme: HttpScheme,
    uri: &'static str,
    port: i32,
    user: &'static str,
    pass: &'static str,
}

impl Connection {
    // this is a constructor basically
    // or initializer for the struct
    // same thing basically
    fn new(
        scheme: HttpScheme,
        uri: &'static str,
        port: i32,
        user: &'static str,
        pass: &'static str,
    ) -> Self {
        Connection {
            scheme,
            uri,
            port,
            user,
            pass,
        }
    }

    fn build_connection_string(&self) -> String {
        let prefix: &str = match self.scheme {
            HttpScheme::HTTP => "http://",
            HttpScheme::HTTPS => "https://",
        };

        // we could also use the !format macro
        // produces the same result
        // let url = format!("{}{}@{}@{}", prefix, self.user, self.pass, self.uri);
        let mut connection_string: String = String::from(prefix);
        connection_string.push_str(self.user);
        connection_string.push_str("@");
        connection_string.push_str(self.pass);
        connection_string.push_str("@");
        connection_string.push_str(self.uri);

        return connection_string;
    }

    // we could define &mut self
    // if we need to manipulate the struct's field
    fn connect(&self) -> () {
        println!("Using:");
        println!(
            "User: {}, Pass: {}, Port: {}, Scheme: {:?}, Uri: {}",
            self.user, self.pass, self.port, self.scheme, self.uri
        );
        println!("Connecting to: {}", self.build_connection_string());
        println!("Connected!");
    }
}
