fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

enum AccountType {
    Boosted,
    Cheater,
    Legit,
}

impl AccountType {
    fn from(is_cheater: bool, is_boosted: bool) -> AccountType {
        if is_cheater && is_boosted {
            AccountType::Boosted
        } else if is_cheater {
            AccountType::Cheater
        } else {
            AccountType::Legit
        }
    }
}

// in practice this should return something like that
// but we can extend it even further with an enum
fn user_is_boosted(is_cheater: bool, is_boosted: bool) -> Option<bool> {
    if is_cheater && is_boosted {
        Some(true)
    } else if is_cheater {
        Some(false)
    } else {
        None
    }
}

fn main() {
    let availability = is_item_in_stock(true, false);

    match availability {
        Option::Some(true) => println!("Yes, the item is available"),
        Option::Some(false) => println!("No, the item is not in stock"),
        Option::None => println!("Your item doesn't exist in our system"),
    }

    // if the result is true then the user is boosted
    if let Some(true) = user_is_boosted(true, true) {
        println!("This user is boosted");
    }

    if let AccountType::Boosted = AccountType::from(true, true) {
        println!("This is a boosted account");
    }
}
