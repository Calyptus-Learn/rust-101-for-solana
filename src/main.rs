mod basics;
mod control_flow;
mod data_structures;

fn main() {
    println!("--- Basics ---");
    basics::demonstrate_unsigned_integers();
    basics::demonstrate_signed_integers();
    basics::demonstrate_floating_point();
    basics::demonstrate_booleans();
    basics::demonstrate_characters();
 

    println!("--- Data Structures ---");
    data_structures::demonstrate_arrays();
    data_structures::demonstrate_vectors();
    data_structures::demonstrate_tuples();
    data_structures::demonstrate_hash_maps();


    println!("--- Control Flow ---");
    control_flow::demonstrate_for_loop();
    control_flow::demonstrate_infinite_loop();
    control_flow::demonstrate_while_loop()

}
