

// The `impl` keyword in Rust is used to define methods on structs (or enums).
// This allows you to define functions that are associated with a particular type.

/// A basic struct representing a Rectangle.
pub struct Rectangle {
    width: u32,
    height: u32,
}

/// Implementing methods for the Rectangle struct.
impl Rectangle {
    /// Creates a new Rectangle.
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    /// Calculates the area of the Rectangle.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Checks if the Rectangle is square.
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
