use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    names.insert("NameOne");
    names.insert("NameTwo");
    names.insert("NameThree");

    if names.contains(&"NameOne") {
        println!("Name One Founded!");
    }
}