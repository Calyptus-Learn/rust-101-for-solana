

// The `match` statement in Rust is a powerful way to compare a value against a series of patterns.
// It's similar to a "switch" statement in other languages but more versatile.

/// Demonstrates the use of `match` with a simple integer.
/// It will match the number to specific cases and print a corresponding message.
pub fn match_number(number: i32) {
    match number {
        1 => println!("You entered one!"),
        2 => println!("You entered two!"),
        3 => println!("You entered three!"),
        _ => println!("You entered a number other than one, two, or three."),
    }
}

/// Demonstrates the use of `match` with a boolean value.
/// It will print a message based on the boolean state.
pub fn match_boolean(value: bool) {
    match value {
        true => println!("The value is true!"),
        false => println!("The value is false!"),
    }
}

pub fn match_colors(color:String){
    match color.as_str() {
        "Yellow" => println!("Color is Yello"),
        _ => print!("Other color than Yellow")
    }
}