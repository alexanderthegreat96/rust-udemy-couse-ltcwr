// aliased primitive here
type Meters = i32;

struct User {
    user_id: i32,
    user_name: &'static str,
}

type Customer = User;

fn main() {
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    let customer: Customer = Customer {
        user_id: 10,
        user_name: "mike",
    };

    println!(
        "Our user is now a customer with: {}({})",
        customer.user_name, customer.user_id
    );
    println!(
        "A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long."
    );
}
