

// Enums in Rust are a way to define a type that can have different values, but these values are restricted to a predefined set of variants.
// Think of enums like a dropdown menu in a form, where you can select only one of the given options.

/// An enum representing traffic light signals.
/// Each variant represents a different state of a traffic light.
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// Function to simulate the behavior of a traffic light.
/// Depending on the light's state, it provides instructions.
pub fn traffic_light_action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready..."),
        TrafficLight::Green => println!("Go!"),
    }
}

// Now, let's move to a simplified Solana-related example.
// Let's imagine Solana transactions are of two types - Pay and Stake.

/// Represents simplified versions of transactions in Solana.
pub enum SolanaTransaction {
    // A payment transaction from one account to another with an amount.
    Pay(String, String, u64),
    // Staking some amount to a validator.
    Stake(String, u64),
}

/// Processes a simplified Solana transaction.
pub fn process_solana_transaction(tx: SolanaTransaction) {
    match tx {
        SolanaTransaction::Pay(from, to, amount) => 
            println!("Paying {} SOL from {} to {}", amount, from, to),
        SolanaTransaction::Stake(validator, amount) => 
            println!("Staking {} SOL to validator {}", amount, validator),
    }
}
