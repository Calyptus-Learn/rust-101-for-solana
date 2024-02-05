fn main() {
    // Unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u32 = 10;

    // Signed integer
    // i8, i16, i32, i64, i128
    let signed = -10;

    // Float is used for decimals
    let float = 0.32;

    // Char is used for a single character
    let character = 'a';

    // Boolean is used for true or false
    let boolean = true;

    // Tuple is used for grouping different data types
    let tuple = (1, -2, 3.0, 4, true);

    // Array is used for grouping the same data types
    let array = [1, 2, 3, 4, 5];

    // String is used for grouping characters
    let string = "Hello World";

    // Vector is used for grouping the same data types, and it is dynamic
    let mut vector = vec![1, 2, 3, 4, 5];
    vector.push(6);

    // Hash map is used for grouping two different data types as key-value pairs
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("Solana", 100);
    hash_map.insert("age", 2);

    // Enums
    enum Color {
        Red,
        Green,
        Blue,
    }

    // Hash set is used for grouping the same data types
    let mut hash_set = std::collections::HashSet::new();
    hash_set.insert("John Doe");
    hash_set.insert("Jane Doe");

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
        height: u32,
    }

    // Add a method to the struct via the impl keyword
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
        side: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    let area_calc = Square { side: 2.0 };
    println!("Calculation for the area of the rectangle using a trait is {}", area_calc.area());

    // Example 11: String Concatenation
    let greeting = "Hello";
    let name = "Alice";
    let full_greeting = format!("{} {}", greeting, name);
    println!("Greeting: {}", full_greeting);

    // Example 12: Conditional Statement (if-else)
    let number = 15;
    if number > 10 {
        println!("Number is greater than 10");
    } else {
        println!("Number is not greater than 10");
    }

    // Example 13: Match Statement (pattern matching)
    let color = Color::Blue;
    match color {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
    }

    // Example 14: Option Type
    let maybe_number: Option<i32> = Some(42);
    match maybe_number {
        Some(value) => println!("The number is: {}", value),
        None => println!("No number provided"),
    }

    // Example 15: Result Type
    fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
        if b == 0.0 {
            return Err("Cannot divide by zero");
        }
        Ok(a / b)
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    // Example 16: Struct with Methods
    struct Circle {
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    let circle = Circle { radius: 5.0 };
    println!("Area of the circle: {}", circle.area());

    // Example 17: Vector Iteration and Transformation
    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared numbers: {:?}", squared_numbers);

    // Example 18: Hash Map Iteration
    for (key, value) in hash_map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    // Example 19: Custom Error Type
    #[derive(Debug)]
    enum MyError {
        DivisionByZero,
        OtherError,
    }

    fn custom_error_example() -> Result<(), MyError> {
        // Some logic that may result in an error
        Err(MyError::OtherError)
    }

    match custom_error_example() {
        Ok(_) => println!("Operation succeeded"),
        Err(e) => println!("Error: {:?}", e),
    }

    // Example 20: Lifetime Annotations
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);
}
