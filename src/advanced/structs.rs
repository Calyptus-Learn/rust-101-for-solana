

// Structs in Rust are used to create custom data types.
// They allow you to group together related data and functionalities.

/// A basic struct in Rust representing a user.
/// It holds data related to a user such as username and email.
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

/// Creates a new User with the given email and username.
pub fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/// Prints details of a User.
pub fn print_user(user: &User) {
    println!("Username: {}, Email: {}, Sign in count: {}, Active: {}", 
             user.username, user.email, user.sign_in_count, user.active);
}

// Transitioning to a more specific example relevant to Solana:

// In Solana, accounts can be thought of as structures holding data and SOL (Solana's cryptocurrency).
// Here, we define a struct to represent a Solana account for demonstration purposes.

/// Represents a Solana account with basic attributes.
pub struct SolanaAccount {
    pub address: String,
    pub balance_sol: f64,
    pub data: Vec<u8>,
    pub is_executable: bool,
}

/// Creates a new Solana account with the specified address and balance.
pub fn create_solana_account(address: String, balance: f64) -> SolanaAccount {
    SolanaAccount {
        address,
        balance_sol: balance,
        data: Vec::new(),
        is_executable: false,
    }
}

/// Prints details of a Solana account.
pub fn print_solana_account(account: &SolanaAccount) {
    println!("Address: {}, Balance: {} SOL, Data Size: {} bytes, Executable: {}",
             account.address, account.balance_sol, account.data.len(), account.is_executable);
}
