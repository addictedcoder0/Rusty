extern crate phrases;
// complex imports
use phrases::english::{greetings,farewells};
use phrases::japanese::{greetings as ja_greetings,farewells as ja_farewells};


fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", ja_greetings::hello());
    println!("Goodbye in Japanese: {}", ja_farewells::goodbye());

    println!("Hello in Hindi: {}", phrases::hindi::greetings::hello());
    println!("Goodbye in Hindi: {}", phrases::hindi::farewells::goodbye());
}

