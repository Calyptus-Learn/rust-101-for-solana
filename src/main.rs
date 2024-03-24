#[allow(dead_code)]
fn main() {
    // unsigned integer
    // u8, u16, u32, u64, u128
    let _unsigned: u32 = 10;

    // signed integer
    // i8, i16, i32, i64, i128
    let _signed = -10;

    // float is used for decimals
    let _float = 0.32;

    // println!("Different numbers => {}, {}, {}", unsigned, signed, float);

    // char is used for single character
    let _character = 'a';
    // println!("Character => {}", character);

    // boolean is used for true or false
    let _boolean = true;
    // println!("Boolean => {}", boolean);

    // tuple is used for grouping different data types
    let _tuple = (1, -2, 3.0, 4, true);
    // println!("Tuple => {:?}", tuple);

    // array is used for grouping same data types
    let _array = [1, 2, 3, 4, 5];
    // println!("Array => {:?}", array);

    // string is used for grouping characters
    let _string = "Hello World";
    // println!("String => {}", string);

    // vector is used for grouping same data types and it is dynamic
    let mut vector = vec![1, 2, 3, 4, 5];
    vector.push(6);
    // println!("Vector => {:?}", vector);

    // hash map is used for grouping 2 different data types as key value pair
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("Solana", 100);
    hash_map.insert("age", 2);
    // println!("Hash Map => {:?}", hash_map);

    // enums

    enum Color {
        Red,
        Green,
        Blue,
    }

    // hash set is used for grouping same data types

    let mut hash_set = std::collections::HashSet::new();

    hash_set.insert("John Doe");
    hash_set.insert("Jane Doe");

    // println!("Hash Set => {:?}", hash_set);

    // Looping

    for i in 0..10 {
        println!("Looping => {}", i);
    }

    let mut i = 0;
    while i < 10 {
        println!("Looping => {}", i);
        i += 1;
    }

    let mut counter = 0;
    loop {
        println!("Looping...");

        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    // Looping over an array
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Array element: {}", element);
    }

    // Looping over an iterator
    let array = [100, 200, 300, 400, 500];
    for (index, value) in array.iter().enumerate() {
        println!("Value at index {}: {}", index, value);
    }

    // Implementation
    
    // Define a struct
    struct Rectangle { 
        width: u32, 
        height: u32
    }
    
    // Add a method to the struct via impl keyword
    impl Rectangle { 
        fn area(&self) -> u32 { 
            self.width * self.height 
        }
    }
    
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {}", rect.area());
    
    // Implementation using trait
    trait HasArea { 
        fn area(&self) -> f64;
    }
    
    struct Square { 
        side: f64
    }
    
    impl HasArea for Square { 
        fn area(&self) -> f64 { 
            self.side * self.side 
        }
    }

    let area_calc = Square {side: 2.0};
    println!("Calculation for the area of the rectangle using a trait is {}", area_calc.area());
}
