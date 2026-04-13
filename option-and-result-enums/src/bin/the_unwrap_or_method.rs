use rand::seq::{IndexedRandom, SliceRandom};

fn main() {
    let present_value = Some(13);
    let missing_value: Option<bool> = None;

    // unwrap or is a default value basically
    // so if a none variant will result in whatever the default is
    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(true));

    let my_number: i32 = pick_random_value(10).unwrap_or(4);
    let my_number_oob: i32 = pick_random_value(100).unwrap_or(4);

    println!("My numbers: {} {}", my_number, my_number_oob);
}

fn pick_random_value(ceil: i32) -> Option<i32> {
    let max: i32 = 20;
    if ceil > max {
        return None;
    }

    let mut rng = rand::rng();

    let mut nums: Vec<i32> = (1..max).collect();
    nums.shuffle(&mut rng);

    let res = nums.choose(&mut rng);
    let chosen = res.unwrap();
    Some(*chosen)
}
