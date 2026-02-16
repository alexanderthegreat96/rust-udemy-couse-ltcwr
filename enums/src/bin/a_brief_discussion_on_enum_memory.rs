#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn main() {
    // memory wise
    // rust choses the enum's memory allocation
    // based on what consumes the most amount of memory from that list
    // considering Paypal stores a double string, that will in turn allocate memory based on that
    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));

    my_payment_method =
        PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("{:?}", my_payment_method);
}
