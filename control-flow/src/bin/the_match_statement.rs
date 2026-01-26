#[derive(Debug, PartialEq)]
enum VehicleType {
    SUV,
    BUS,
    MINIVAN,
    OFFROAD,
    SEDAN,
}

#[derive(Debug, PartialEq)]
enum VehicleColor {
    BLACK,
    WHITE,
    BLUE,
    ORANGE,
    GREEN,
    RED,
}

impl VehicleType {
    fn from_str(input: &str) -> Option<Self> {
        match input.to_uppercase().as_str() {
            "SUV" => Some(Self::SUV),
            "BUS" => Some(Self::BUS),
            "MINIVAN" => Some(Self::MINIVAN),
            "OFFROAD" => Some(Self::OFFROAD),
            "SEDAN" => Some(Self::SEDAN),
            _ => None,
        }
    }
}

impl VehicleColor {
    fn from_str(input: &str) -> Option<Self> {
        match input.to_uppercase().as_str() {
            "BLACK" => Some(Self::BLACK),
            "WHITE" => Some(Self::WHITE),
            "BLUE" => Some(Self::BLUE),
            "ORANGE" => Some(Self::ORANGE),
            "GREEN" => Some(Self::GREEN),
            "RED" => Some(Self::RED),
            _ => None,
        }
    }
}
#[derive(Debug)]
struct Make {
    make_id: i32,
    make_name: String,
}
#[derive(Debug)]
struct Model {
    model_id: i32,
    model_name: String,
}

impl Make {
    fn new(id: i32, name: String) -> Self {
        Self {
            make_id: id,
            make_name: name,
        }
    }
}

impl Model {
    fn new(id: i32, name: String) -> Self {
        Self {
            model_id: id,
            model_name: name,
        }
    }
}
#[derive(Debug)]
struct Car {
    make: Make,
    model: Model,
    vehicle_type: VehicleType,
    color: VehicleColor,
    engine_cc: f64,
    top_speed: f64,
}

impl Car {
    fn new(
        make_id: i32,
        make_name: String,
        model_id: i32,
        model_name: String,
        vehicle_type: &str,
        vehicle_color: &str,
        engine_cc: f64,
        top_speed: f64,
    ) -> Option<Self> {
        Some(Self {
            make: Make::new(make_id, make_name),
            model: Model::new(model_id, model_name),
            vehicle_type: VehicleType::from_str(vehicle_type)?,
            color: VehicleColor::from_str(vehicle_color)?,
            engine_cc,
            top_speed,
        })
    }
}

fn main() {
    // match statement
    // works like a switch statement
    // reacts to multiple possible variants
    // of the same value

    // rust ensurees what we cover
    // every possible value
    // in the match pattern
    let evaluation = true;

    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("the value is false");
        }
    }

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("{value}");

    let car: Option<Car> = Car::new(
        1,
        String::from("BMW"),
        1,
        String::from("X5"),
        "SUV",
        "black",
        4.565,
        175.2,
    );

    dbg!(car);
}
