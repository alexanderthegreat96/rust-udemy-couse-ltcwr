/*
Define a Tier enum with three variants: Gold, Silver,
Platinum. Derive a Debug implementation for the Tier enum.

Define a Subscription enum with three variants: Free,
Basic, and Premium. Derive a Debug implementation for the
Subscription enum.

The Free variant should have no associated data.

The Basic variant should be a tuple variant with two pieces
of data. The first one should be a f64 (the price per month)
and the second one should be a u32 (the number of months).

The Premium variant should be a struct variant with a 'tier'
field. The tier field should be a Tier enum value.

Define a 'summarize' method on the Subscription enum.

If the Subscription is Free, output the text "You have
limited access to the site".

If the Subscription is Basic, output the text "You have
limited access to the site's premium features for {price}
for {months} months", where {price} amd {months} come from
the tuple variant's associated data.

If the Subscription is Premium, output the text "You have
full access to the site's premium features. Your tier is
{tier:?}"", where {tier} comes from the struct variant's
associated 'tier' field.

In the `main` function, create 3 instances of Subscription,
each one with a different variant. Invoke the `summarize`
method on each instance.
*/

#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),        // price and months
    Premium { tier: Tier }, // enum struct variant
}

impl Subscription {
    fn summarize(self) -> () {
        match self {
            Subscription::Free => {
                println!("You're on the free plan!");
            }
            Subscription::Basic(price, months) => {
                println!(
                    "You havelimited access to the site's premium features for {price}for {months} month"
                );
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                );
            }
        }
    }
}
fn main() {
    let free_sub: Subscription = Subscription::Free;
    let basic_sub: Subscription = Subscription::Basic(14.99, 10);
    let platinum: Subscription = Subscription::Premium {
        tier: Tier::Platinum,
    };
    let gold: Subscription = Subscription::Premium { tier: Tier::Gold };
    let silver: Subscription = Subscription::Premium { tier: Tier::Silver };

    free_sub.summarize();
    basic_sub.summarize();
    platinum.summarize();
    gold.summarize();
    silver.summarize();
}
