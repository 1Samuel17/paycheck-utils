//! Module for handling paycheck income calculations for hourly paid employees
//! This module calculates from a bi-weekly paycheck perspective to synthesize how an employee thinks about, views, and plans their income.

use crate::constants::*;
use crate::utils::round_2_decimals;

/// Determine gross bi-weekly paycheck based on hourly rate and hours worked per week
/// # Arguments
/// * `rate` - hourly pay rate
/// * `hours_per_week` - number of hours worked per week
/// # Returns
/// * `f32` - gross bi-weekly paycheck amount rounded to 2 decimal places
/// # Example
/// ```
/// use paycheck_utils::income::determine_gross_paycheck;
/// use paycheck_utils::round_2_decimals;
///
/// let gross_paycheck = determine_gross_paycheck(20.0, 45.0);
/// assert_eq!(gross_paycheck, 1900.00);
/// ```
/// # Notes
/// * Overtime is calculated at time and a half for hours worked over 40 hours per week
/// * Standard hours are capped at 40 hours per week for regular pay calculation
/// * Bi-weekly paycheck is calculated over 2 week pay periods
pub fn determine_gross_paycheck(rate: f32, hours_per_week: f32) -> f32 {
    let regular_hours = if hours_per_week > STANDARD_HOURS_PER_WEEK {
        STANDARD_HOURS_PER_WEEK
    } else {
        hours_per_week
    };

    let overtime_hours = if hours_per_week > STANDARD_HOURS_PER_WEEK {
        hours_per_week - STANDARD_HOURS_PER_WEEK
    } else {
        0.0
    };

    let gross_paycheck =
        ((regular_hours * rate) + (overtime_hours * (rate * OVERTIME_MULTIPLIER))) * PAY_PERIOD;

    round_2_decimals(gross_paycheck)
}

// UNIT TESTS FOR INCOME MODULE

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_determine_gross_paycheck() {
        let rate = 20.0;
        let hours_per_week = 45.0;
        let gross_paycheck = determine_gross_paycheck(rate, hours_per_week);
        assert_eq!(gross_paycheck, 1900.00);
    }

    #[test]
    fn test_determine_gross_paycheck_no_overtime() {
        let rate = 15.0;
        let hours_per_week = 35.0;
        let gross_paycheck = determine_gross_paycheck(rate, hours_per_week);
        assert_eq!(gross_paycheck, 1050.00);
    }
}
