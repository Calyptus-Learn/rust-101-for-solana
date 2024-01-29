mod basics;
mod control_flow;
mod data_structures;
mod advanced;

fn main() {
    println!("--- Basics ---");
    basics::demonstrate_unsigned_integers();
    basics::demonstrate_signed_integers();
    basics::demonstrate_floating_point();
    basics::demonstrate_tbooleans();
    basics::demonstrate_characters();
    basics::demonstrate_fbooleans();
 

    println!("--- Data Structures ---");
    data_structures::demonstrate_arrays();
    data_structures::demonstrate_vectors();
    data_structures::demonstrate_tuples();
    data_structures::demonstrate_hash_maps();


    println!("--- Control Flow ---");
    control_flow::demonstrate_for_loop();
    control_flow::demonstrate_infinite_loop();
    control_flow::demonstrate_while_loop();

 

    println!("--- Structs in Rust: General Example ---");

    let user = advanced::structs::create_user("user@example.com".to_string(), "user123".to_string());
    advanced::structs::print_user(&user);

    println!("\n--- Structs in Rust: Solana Example ---");

    let solana_account = advanced::structs::create_solana_account("123xyz".to_string(), 1000.0);
    advanced::structs::print_solana_account(&solana_account);

    println!("Traffic Light Example:");
    let light = advanced::enums::TrafficLight::Red;
    advanced::enums::traffic_light_action(light);

    println!("\nSolana Transaction Example:");
    let payment = advanced::enums::SolanaTransaction::Pay("Alice".to_string(), "Bob".to_string(), 100);
    advanced::enums::process_solana_transaction(payment);



     println!("--- Match Examples ---");

  
     advanced::match_example::match_number(2);
 
    
     advanced::match_example::match_boolean(true);

     println!("--- Impl Example ---");

  
    let rect = advanced::impl_examples::Rectangle::new(10, 20);

 

    println!("Rectangle area is {}", rect.area());
    println!("Is the rectangle a square? {}", rect.is_square());


    println!("--- Error Handling Example ---");

    let result = advanced::error_handling::safe_divide(10.0, 0.0);

    match result {
        Ok(value) => println!("Division successful: {}", value),
        Err(error_message) => println!("Error: {}", error_message),
    }

    println!("--- Debug trait example---");

    advanced::debug_trait::demonstrate_debug_trait();

   

   
}
