pub mod ops;
pub mod prelude;
pub mod macros;

use serde::{Serialize, Deserialize};

/// A structure to represent a `BigNumber` with integer and fractional parts.
///
/// # Fields
/// - `int`: A vector of `u8` representing the integer part of the number.
/// - `frac`: A vector of `u8` representing the fractional part of the number.
/// - `sign`: A boolean to represent the sign of the number (`true` for positive, `false` for negative).
#[derive(Debug, Serialize, Deserialize)]
pub struct BigNumber {
    pub int: Vec<u8>,
    pub frac: Vec<u8>,
    pub sign: bool,
}

impl BigNumber {
    /// Creates a new `BigNumber` from its raw components.
    ///
    /// # Arguments
    ///
    /// * `int` - A vector of `u8` representing the integer part.
    /// * `frac` - A vector of `u8` representing the fractional part.
    /// * `sign` - A boolean for the sign (`true` for positive, `false` for negative).
    ///
    /// # Returns
    ///
    /// A new `BigNumber` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let int_part = vec![1, 2, 3];
    /// let frac_part = vec![4, 5, 6];
    /// let sign = true;
    /// let big_number = BigNumber::from_raw(int_part, frac_part, sign);
    /// ```
    pub fn from_raw(int: Vec<u8>, frac: Vec<u8>, sign: bool) -> Self {
        Self { int, frac, sign }
    }
}

impl Into<String> for BigNumber {
    /// Converts a `BigNumber` into its `String` representation.
    ///
    /// # Returns
    ///
    /// A string in the format of `+integer.fraction` or `-integer.fraction` depending on the sign.
    ///
    /// # Example
    ///
    /// ```
    /// let big_number = BigNumber::from_raw(vec![1, 2, 3], vec![4, 5, 6], true);
    /// let big_number_string: String = big_number.into();
    /// assert_eq!(big_number_string, "+123.456");
    /// ```
    fn into(self) -> String {
        format!(
            "{}{}.{}",
            match self.sign {
                true => "+",
                false => "-"
            },
            self.int.iter().map(|x| x.to_string()).collect::<String>(),
            self.frac.iter().map(|x| x.to_string()).collect::<String>(),
        )
    }
}

impl From<String> for BigNumber {
    /// Creates a `BigNumber` from a `String` representation.
    ///
    /// # Arguments
    ///
    /// * `value` - A string in the format `+integer.fraction` or `-integer.fraction`.
    ///
    /// # Returns
    ///
    /// A new `BigNumber` instance.
    ///
    /// # Example
    ///
    /// ```
    /// let string_value = "+123.456".to_string();
    /// let big_number: BigNumber = BigNumber::from(string_value);
    /// ```
    fn from(value: String) -> Self {
        // Placeholder implementation for now
        Self {
            int: Vec::new(),
            frac: Vec::new(),
            sign: bool::default(),
        }
    }
}
