// module for handling estimated federal paycheck withholdings using the method outlined by the IRS in Publication 15T (2026) (percentage method)

use crate::utils::*;
use crate::income::*;

// 1. annualize paycheck
// 2. adjust for w4 deductions and credits
// 3. apply tax brackets
// 4. convert back to per-paycheck

pub fn estimate_federal_tax_withholdings(gross_annual_income: f32, filing_status: FilingStatus) -> f32 {
    match filing_status {
        Single => {
            0.0
        },

    }
}







