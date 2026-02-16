#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String),
}

fn main() {
    // we can specify the data type for that specific value
    // same initialization technique, just we add in paranthesis the data type
    let visa = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    let mastercard = PaymentMethodType::DebitCard(String::from("2532-1298-2093-4800"));
    println!("{:?}", visa);
    println!("{:?}", mastercard);
}
