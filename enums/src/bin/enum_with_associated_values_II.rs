#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn main() {
    // different enum variants can store multiple variants of data
    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));

    // we're storing an email and a password
    my_payment_method =
        PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", my_payment_method);
}
