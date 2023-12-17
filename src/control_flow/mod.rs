// This file covers different types of loops in Rust like 'for', 'while', and 'loop'.

// For Loop
pub fn demonstrate_for_loop() {
    for i in 0..10 {
        println!("For Loop => {}", i);
    }
}

// While Loop
pub fn demonstrate_while_loop() {
    let mut i = 0;
    while i < 10 {
        println!("While Loop => {}", i);
        i += 1;
    }
}

// Infinite Loop
pub fn demonstrate_infinite_loop() {
    let mut counter = 0;
    loop {
        println!("Infinite Loop...");
        counter += 1;
        if counter >= 5 {
            break;
        }
    }
}
