#[allow(dead_code)]
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
#[derive(Debug)]
struct Card {
    rank: String,
    suit: CardSuit,
}

fn main() {
    // enum is a type that represents a set of possible values
    // each possible value is called variant
    // a very small example: HttpRequest with 2 values: GET, PUT, POST, DELETE
    let first_card = CardSuit::Hearts;
    println!("{:?}", first_card);

    let mut second_card: CardSuit = CardSuit::Spades;
    println!("{:?}", second_card);

    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    println!("An array of CardSuit: {:?}", card_suits);

    let card_suits = (CardSuit::Hearts, CardSuit::Spades);
    println!("A tuple of CardSuit: {:?}", card_suits);

    let card: Card = Card {
        rank: String::from("General"),
        suit: second_card,
    };

    println!("{:?}", card);
}
