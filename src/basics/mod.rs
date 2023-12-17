// This file covers basic data types in Rust.
// Rust has several primitive data types which include integers, floating-point numbers, booleans, and characters.

// Unsigned Integers: u8, u16, u32, u64, u128
pub fn demonstrate_unsigned_integers() {
    let unsigned: u32 = 10;
    println!("Unsigned integer: {}", unsigned);
}

// Signed Integers: i8, i16, i32, i64, i128
pub fn demonstrate_signed_integers() {
    let signed = -10i32;
    println!("Signed integer: {}", signed);
}

// Floating-Point Numbers: f32, f64
pub fn demonstrate_floating_point() {
    let float = 0.32f64;
    println!("Floating-point number: {}", float);
}

// Characters
pub fn demonstrate_characters() {
    let character = 'a';
    println!("Character: {}", character);
}

// Booleans
pub fn demonstrate_booleans() {
    let boolean = true;
    println!("Boolean: {}", boolean);
}
