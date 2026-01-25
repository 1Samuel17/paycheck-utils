/// Module for handling paycheck income calculations
/// When considering hourly income, this module calculates from a bi-weekly paycheck perspective to synthesize how an employee thinks, views, and plans their income.
/// For salaried income, calculations are more straightforward as they can be directly divided into pay periods, months, and years.

use crate::utils::*;

// per paycheck

pub fn determine_gross_paycheck(rate: f32, hours_per_week: f32) -> f32 {

    let regular_hours = 
        if hours_per_week > STANDARD_HOURS_PER_WEEK {
            STANDARD_HOURS_PER_WEEK
        }
        else {hours_per_week};

    let overtime_hours = 
        if hours_per_week > STANDARD_HOURS_PER_WEEK {
            hours_per_week - STANDARD_HOURS_PER_WEEK
        }
        else {0.0};
    
    let gross_paycheck = ((regular_hours * rate) + (overtime_hours * rate * OVERTIME_MULTIPLIER)) * PAY_PERIOD;

    round_2_decimals(gross_paycheck)
}

// helper functions
fn round_2_decimals(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}


// Quick tests while developing (still need to create more test cases to be more thorough)