// Publishing a Crate to Crates.io
// https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html

// Test commands:
// > cargo doc --open
// > cargo test

//! # Rust Book Publish
//!
//! `rust_book_publish` is a test crate for the Publishing a Crate to Crates section
//! of The Rust Book.
//!
//! See also: https://github.com/lorancou/rust-book-practice

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rust_book_publish::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Errors
///
/// Will panic if `x` is `i32::MAX`.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    #[should_panic]
    fn overflow() {
        add_one(i32::MAX);
    }
}

// Exporting a Convenient Public API with pub use
// [using the same crate, unlike the book]

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
