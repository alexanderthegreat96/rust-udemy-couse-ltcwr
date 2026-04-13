#![allow(dead_code)]

enum Car {
    BMW_X5,
    FORD_MUSTANG,
    ROLLS_ROYCE,
    THORNTON_A1,
}

fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(2);
    play(bass);
    println!("{:?}", bass);

    let invalid_instrument = musical_instruments.get(100);
    play(invalid_instrument);

    let cars: Vec<Car> = Vec::from([
        Car::BMW_X5,
        Car::FORD_MUSTANG,
        Car::ROLLS_ROYCE,
        Car::THORNTON_A1,
    ]);

    for car in cars {
        drive(Some(car));
    }

    drive(None);
    play(Some(String::from("guitar")).as_ref());
}

// so we're mathing whatever we're playing
fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}

fn drive(what: Option<Car>) {
    match what {
        Option::Some(car) => match car {
            Car::BMW_X5 => {
                println!("You're driving a BMW X5");
            }
            Car::FORD_MUSTANG => {
                println!("You're driving a Ford Mustang");
            }
            Car::ROLLS_ROYCE => {
                println!("You're a rich MF, you're driving a Rolls Royce.");
            }
            Car::THORNTON_A1 => {
                println!("Only true Cyberpunk Fans drive the Thornton A1");
            }
        },
        Option::None => {
            println!("You're too poor to afford a card. So you're gonna walk.");
        }
    }
}
