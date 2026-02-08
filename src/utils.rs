//! Utility functions for paycheck calculations and formatting.
//!This module includes functions for rounding values to 2 decimal places and formatting output for display.

/// Rounds a f32 value to 2 decimal places
/// # Arguments
/// * `value` - f32 value to be rounded
/// # Returns
/// * `f32` - rounded value to 2 decimal places
/// # Example
/// ```
/// use paycheck_utils::utils::round_2_decimals;
///
/// let rounded_value = round_2_decimals(123.4567);
/// assert_eq!(rounded_value, 123.46);
/// ```
/// # Notes
/// * Uses standard rounding rules
///
pub fn round_2_decimals(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}

use std::any::{Any, TypeId};
use std::str::FromStr;

pub fn check_converted_value<T: Any + FromStr>(result: &Result<T, T::Err>, expected_type: TypeId) -> bool {
    match result {
        Ok(value) => value.type_id() == expected_type,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_2_decimals() {
        assert_eq!(round_2_decimals(123.4567), 123.46);
        assert_eq!(round_2_decimals(123.451), 123.45);
        assert_eq!(round_2_decimals(123.455), 123.46);
        assert_eq!(round_2_decimals(123.454), 123.45);
    }

    #[test]
    fn test_check_converted_value() {
        let result_ok: Result<f32, _> = "123.45".parse::<f32>();
        let result_err: Result<f32, _> = "abc".parse::<f32>();
        assert!(check_converted_value(&result_ok, TypeId::of::<f32>()));
        assert!(!check_converted_value(&result_err, TypeId::of::<f32>()));
    }
}


