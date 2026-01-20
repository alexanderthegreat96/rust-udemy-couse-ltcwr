// we have to derive ParialEq
// in order to use direct comparison
#[derive(Debug, PartialEq)]
enum Gender {
    M,
    F,
    N,
}

struct User {
    user_id: i32,
    user_name: String,
    gender: Gender,
    age: i16,
}

// i know return is optional
// but i dont care
impl User {
    pub fn new(user_id: i32, user_name: String, gender: Gender, age: i16) -> User {
        return User {
            user_id,
            user_name,
            gender,
            age,
        };
    }

    pub fn is_male(&self) -> bool {
        return self.gender == Gender::M;
    }

    pub fn is_female(&self) -> bool {
        return self.gender == Gender::F;
    }

    pub fn is_non_binary(&self) -> bool {
        return self.gender == Gender::N;
    }

    pub fn is_over_fourty(&self) -> bool {
        return self.age > 40;
    }
}

fn main() {
    let male: User = User::new(123, String::from("Alex"), Gender::M, 30);
    let female: User = User::new(321, String::from("Carol"), Gender::F, 36);
    let older: User = User::new(321, String::from("Nada"), Gender::N, 42);

    if male.is_male() {
        println!("{} is male. He is {} years old.", male.user_name, male.age);
    }

    if female.is_female() {
        println!(
            "{} is female. She is {} years old.",
            female.user_name, female.age
        );
    }

    if older.is_non_binary() {
        println!(
            "{} is non-binary, aka mentally ill, but they are {} years old.",
            older.user_name, older.age
        )
    }

    if older.is_over_fourty() {
        println!("{} is over forty (Age: {}).", older.user_name, older.age);
    }

    let users = vec![male, female, older];

    println!("\n--- User Directory ---");
    for user in &users {
        let category = if user.is_over_fourty() {
            "Senior"
        } else {
            "Junior"
        };

        println!(
            "ID: {} | Name: {} | Category: {}",
            user.user_id, user.user_name, category
        );
    }

    let age: i32 = -40;
    let is_young: bool = age < 35;

    println!("{} {} {}", age.is_positive(), age.is_negative(), is_young);
}
