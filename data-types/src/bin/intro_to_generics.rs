struct Address {
    street: String,
    zip_code: i32,
}
struct User {
    user_id: i32,
    user_name: String,
    age: i32,
    address: Address,
}

impl Address {
    fn new(street: String, zip_code: i32) -> Address {
        return Address { street, zip_code };
    }

    fn get_address(&self) -> String {
        let street: &String = &self.street;
        let zip_code: &i32 = &self.zip_code;

        return format!("{}, {}", street, zip_code);
    }
}

impl User {
    fn new(user_id: i32, user_name: String, age: i32, address: Address) -> User {
        return User {
            user_id,
            user_name,
            age,
            address,
        };
    }

    fn get_user_info(&self) -> String {
        return format!(
            "{}. {} -> age: {}, address: {}",
            &self.user_id,
            &self.user_name,
            &self.age,
            &self.address.get_address()
        );
    }
}

fn main() {
    // generics and generics in rust
    // a generic is a type argument
    // when providing it, we're essentially
    // expecting that type to be provided or returned
    // with these arguments
    // we're using generics for type predictions
    let month_days: std::ops::Range<i32> = 1..31;
    let letters: std::ops::Range<char> = 'b'..'f';
    // Value - 5
    // Type - i32

    dbg!(month_days);
    dbg!(letters);

    let mut users: Vec<User> = Vec::new();

    users.push(User::new(
        1,
        String::from("alex123"),
        30,
        Address::new(String::from("upperside 122"), 75643),
    ));

    users.push(User::new(
        2,
        String::from("jane122"),
        37,
        Address::new(String::from("lower eastside CA"), 587423),
    ));

    for u in users {
        println!("{}", u.get_user_info());
    }
}
