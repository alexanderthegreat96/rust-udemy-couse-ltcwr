use std::collections::HashMap;

/*
Bring the HashMap type into the current's file's namespace.

Declare a `sauces_to_meals` HashMap. The keys will be
string slices and the values will be a vector of string
slices. Use the `from` function to populate the HashMap
with 2 key-value pairs:

Key: "Ketchup"
Value: Vector of ["French Fries", "Burgers", "Hot Dogs"]

Key: "Mayonnaise"
Value: Vector of ["Sandwiches", "Burgers", "Coleslaw"]

Use the `insert` method to add the following key-value
pair to the HashMap.

Key: "Mustard"
Value: Vector of ["Hot dog", "Burgers", "Pretzels"]

Use the `remove` method to remove the key-value pair
where "Mayonnaise" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Use the `get` method to retrieve the key-value pair
where "Mustard" is the key. Find a way to retrieve
the vector inside the Option and print it out.

Use the `entry` and `or_insert` methods to add the
following key-value pair:

Key: "Soy Sauce"
Value: Vector of ["Sushi", "Dumplings"]

Finally, print out the final `sauces_to_meals` HashMap.

The final result should be:
{
  "Ketchup": ["French Fries", "Burgers", "Hot Dogs"],
  "Soy Sauce": ["Sushi", "Dumplings"],
  "Mustard": ["Hot dog", "Burgers", "Pretzels"]
}
*/
fn main() {
    let values: [(&str, Vec<&str>); 2] = [
        ("Ketchup", ["French Fries", "Burgers", "Hotdogs"].to_vec()),
        (
            "Mayonnaise",
            ["Sandwitches", "Burgers", "Coleslaw"].to_vec(),
        ),
    ];

    let mut sauces_and_meals: HashMap<&str, Vec<&str>> = HashMap::from(values);
    sauces_and_meals.insert("Mustard", ["Hot Dog", "Burgers", "Pretzels"].to_vec());
    sauces_and_meals.remove("Mayonnaise");

    let mustard_stuff: Option<&Vec<&str>> = sauces_and_meals.get("Mustard");
    match mustard_stuff {
        Some(meals) => {
            println!("Meals using Mustard: {:?}", meals);
        }
        None => {
            println!("Unable to find anything that uses mustard");
        }
    }

    sauces_and_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!(
        "All of our main incredients + meals: {:#?}",
        sauces_and_meals
    );
}
