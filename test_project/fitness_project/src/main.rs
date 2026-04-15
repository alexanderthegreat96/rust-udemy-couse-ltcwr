use fitness_project::{CardioExercise, CardioTool, GymWorkout, WeightliftingExercise};

fn main() {
    let cardio_exercise: CardioExercise =
        CardioExercise::new("Monday".to_string(), CardioTool::Bike, 60);

    let weightlifting_exercise: WeightliftingExercise =
        WeightliftingExercise::new("Tuesday".to_string(), 10);

    let workout: GymWorkout = GymWorkout::new(cardio_exercise, weightlifting_exercise);
    println!("{:#?}", workout);
}
