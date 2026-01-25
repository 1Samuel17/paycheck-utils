// module for handling estimated federal paycheck withholdings using the method outlined by the IRS in Publication 15T (2026) (percentage method)

use std::sync::BarrierWaitResult;

use crate::utils::*;
use crate::income::*;

// 1. annualize paycheck
// 2. adjust for w4 deductions and credits
// 3. apply tax brackets
// 4. convert back to per-paycheck

pub fn estimate_paycheck_federal_withholdings(gross_paycheck: f32, filing_status: FilingStatus, elected_pretax_deductions: f32) -> f32 {
    
    let gross_annualized_paycheck = gross_paycheck * PAY_PERIODS_PER_YEAR;

    let standard_deduction = 
    match filing_status {
        Single => {SINGLE_DEDUCTION},
        HeadOfHousehold => {HEAD_OF_HOUSEHOLD_DEDUCTION}, // for future implementation
        MarriedFilingJointly => {MARRIED_FILING_JOINTLY_DEDUCTION}, // for future implementation
        MarriedFilingSeparate => {MARRIED_FILING_SEPERATE_DEDUCTION} // for future implementation
    };

    let adjusted_annualized_paycheck = gross_annualized_paycheck - standard_deduction - elected_pretax_deductions;

    let estimated_annual_withholdings = apply_tax_brackets(adjusted_annualized_paycheck);

    let estimated_paycheck_withholdings = estimated_annual_withholdings / PAY_PERIODS_PER_YEAR;

    estimated_paycheck_withholdings
}

fn apply_tax_brackets(adjusted_annualized_paycheck: f32) -> f32 {

    if adjusted_annualized_paycheck > SINGLE_BRACKET_6_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_6_THRESHOLD) * TAX_BRACKET_7_RATE) + (SINGLE_BRACKET_6_THRESHOLD * TAX_BRACKET_6_RATE)

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_5_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_5_THRESHOLD) * TAX_BRACKET_6_RATE) + (SINGLE_BRACKET_5_THRESHOLD * TAX_BRACKET_5_RATE)

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_4_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_4_THRESHOLD) * TAX_BRACKET_5_RATE) + (SINGLE_BRACKET_4_THRESHOLD * TAX_BRACKET_4_RATE)

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_3_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_3_THRESHOLD) * TAX_BRACKET_4_RATE) + (SINGLE_BRACKET_3_THRESHOLD * TAX_BRACKET_3_RATE)

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_2_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_2_THRESHOLD) * TAX_BRACKET_3_RATE) + (SINGLE_BRACKET_2_THRESHOLD * TAX_BRACKET_2_RATE)

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_1_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_1_THRESHOLD) * TAX_BRACKET_2_RATE) + (SINGLE_BRACKET_1_THRESHOLD * TAX_BRACKET_1_RATE)

    } else {
        adjusted_annualized_paycheck * TAX_BRACKET_1_RATE
    }
    
}





