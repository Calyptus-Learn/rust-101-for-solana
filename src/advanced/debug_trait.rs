#[derive(Debug)]

struct Person {
    name: String,
    age: u8
}

pub fn demonstrate_debug_trait() {
    let person: Person = Person {
        name: "Inaam".to_string(),
        age: 28
    };

    println!("Debug trait : {:?}",person )
}

