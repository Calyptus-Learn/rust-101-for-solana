

// In Rust, error handling is primarily done using the Result type.
// Result is an enum with variants Ok(T) for success and Err(E) for failure.

/// Tries to divide two numbers and returns a Result.
/// Returns Ok with the result if successful, or an Err with a message if not.
pub fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        // Using Err to return an error message
        Err(String::from("Error: Cannot divide by zero."))
    } else {
        // Using Ok to return the result
        Ok(numerator / denominator)
    }
}
