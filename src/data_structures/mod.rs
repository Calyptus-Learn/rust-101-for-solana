// This file explains various collections in Rust like arrays, vectors, tuples, and hash maps.

// Arrays
pub fn demonstrate_arrays() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
}

// Vectors
pub fn demonstrate_vectors() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    vector.push(6);
    println!("Vector: {:?}", vector);
}

// Tuples
pub fn demonstrate_tuples() {
    let tuple: (i32, i32, f64, i32, bool) = (1, -2, 3.0, 4, true);
    println!("Tuple: {:?}", tuple);
}

// Hash Maps
pub fn demonstrate_hash_maps() {
    let mut hash_map: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    hash_map.insert("Solana", 100);
    hash_map.insert("age", 2);
    println!("Hash Map: {:?}", hash_map);
}
