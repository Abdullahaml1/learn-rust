//! # playing-with-cargo
//! `playing-with-cargo` is a crate to test and play with carog io
//! very good productive way of writing code !
/// Adds two numbers together
///
/// # Examples
///
/// ```
/// let left = 5;
/// let right = 6;
/// let answer = playing_with_cargo::add(left, right);
/// assert_eq!(answer, 11);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// exposing these types as from front of the crate
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
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
