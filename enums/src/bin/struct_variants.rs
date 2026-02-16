#[allow(dead_code)]
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal { username: String, password: String },
    Data(User),
    Cash,
}

impl PaymentMethodType {
    // Helper to identify the method type quickly
    fn get_label(&self) -> &str {
        match self {
            Self::CreditCard(_) => "Credit Card",
            Self::DebitCard(_) => "Debit Card",
            Self::PayPal { .. } => "PayPal",
            Self::Data(_) => "User Data",
            Self::Cash => "Cash",
        }
    }

    // Helper to safely extract a username/name if available
    fn get_account_name(&self) -> Option<String> {
        match self {
            Self::PayPal { username, .. } => Some(username.clone()),
            Self::Data(user) => Some(user.name.clone()),
            _ => None,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("0012-3456"));
    let cc = PaymentMethodType::CreditCard(String::from("2342ji3-234-23-4-23-4"));

    // struct variant: easier to use than regular associated values
    let paypal = PaymentMethodType::PayPal {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };

    // tuple variant wrapping a struct
    let data = PaymentMethodType::Data(User {
        name: String::from("someone"),
        age: 33,
    });

    // --- Testing Access ---

    // 1. Using the impl helper
    println!("Method 1 is a: {}", paypal.get_label());
    if let Some(name) = paypal.get_account_name() {
        println!("Account: {}", name);
    }

    // 2. Direct destructuring of the struct variant
    if let PaymentMethodType::PayPal { username, password } = &paypal {
        println!("Extracted: {} with pass: {}", username, password);
    }

    // 3. Deep destructuring of the Data variant
    if let PaymentMethodType::Data(User { name, age }) = &data {
        println!("User inside Data: {} (Age: {})", name, age);
    }

    println!("\nDebug Print:");
    println!("{:?}", paypal);
    println!("{:?}", data);
}
