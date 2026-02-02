fn main() {
    // strinct literal refferences implement the copy trait
    // which enables you to essentially copy the content
    // of the variable somewhere else
    let ice_cream = "Cookies and Cream";
    // desert copiesm the icecream
    let dessert = ice_cream;
    println!("{ice_cream} {dessert}.");
}
