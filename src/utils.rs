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

// pub fn center_100(text: &str) {
//     // future implementation of center alignment and line wrapping for displaying
//     ()
// }