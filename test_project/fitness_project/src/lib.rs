#![allow(unused)]
pub mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {}", self::NUTRITIONIST);
    }
}

pub mod cardio;
pub mod weightlifting;

pub use cardio::CardioTool;
pub use cardio::Exercise as CardioExercise;
pub use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new(cardio: CardioExercise, weightlifting: WeightliftingExercise) -> Self {
        diet::ask_about_program();
        cardio::ask_about_program();
        weightlifting::ask_about_program();

        Self {
            cardio,
            weightlifting,
        }
    }
}
