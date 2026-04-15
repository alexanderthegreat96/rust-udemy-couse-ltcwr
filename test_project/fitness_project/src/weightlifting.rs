const PERSONAL_TRAINER: &str = "Will Weight";

pub fn ask_about_program() {
    println!("The weightlifting trainer is {}", self::PERSONAL_TRAINER);
}

#[derive(Debug)]
pub struct Exercise {
    name: String,
    reps: u32,
}

impl Exercise {
    pub fn new(name: String, reps: u32) -> Self {
        Self { name, reps }
    }
}
