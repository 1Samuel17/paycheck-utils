// module for handling estimated federal paycheck withholdings using the method outlined by the IRS in Publication 15T (2026) (percentage method)

use crate::utils::*;

// 1. annualize paycheck
// 2. adjust for w4 deductions and credits
// 3. apply tax brackets
// 4. convert back to per-paycheck

pub fn estimate_paycheck_federal_withholdings(gross_paycheck: f32, filing_status: FilingStatus, elected_pretax_deductions: f32) -> f32 {
    
    let gross_annualized_paycheck = gross_paycheck * PAY_PERIODS_PER_YEAR;

    let standard_deduction = 
    match filing_status {
        FilingStatus::Single => {SINGLE_DEDUCTION},
        FilingStatus::HeadOfHousehold => {HEAD_OF_HOUSEHOLD_DEDUCTION}, // for future implementation
        FilingStatus::MarriedFilingJointly => {MARRIED_FILING_JOINTLY_DEDUCTION}, // for future implementation
        FilingStatus::MarriedFilingSeparate => {MARRIED_FILING_SEPERATE_DEDUCTION} // for future implementation
    };

    let adjusted_annualized_paycheck = gross_annualized_paycheck - standard_deduction - elected_pretax_deductions;

    let estimated_annual_withholdings = apply_tax_brackets(adjusted_annualized_paycheck);

    estimated_annual_withholdings / PAY_PERIODS_PER_YEAR // estimated per-paycheck federal withholding
}

fn apply_tax_brackets(adjusted_annualized_paycheck: f32) -> f32 {

    if adjusted_annualized_paycheck > SINGLE_BRACKET_6_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_6_THRESHOLD) * TAX_BRACKET_7_RATE) + SINGLE_BRACKET_7_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_5_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_5_THRESHOLD) * TAX_BRACKET_6_RATE) + SINGLE_BRACKET_6_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_4_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_4_THRESHOLD) * TAX_BRACKET_5_RATE) + SINGLE_BRACKET_5_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_3_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_3_THRESHOLD) * TAX_BRACKET_4_RATE) + SINGLE_BRACKET_4_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_2_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_2_THRESHOLD) * TAX_BRACKET_3_RATE) + SINGLE_BRACKET_3_BASE_TAX

    } else if adjusted_annualized_paycheck > SINGLE_BRACKET_1_THRESHOLD {
        ((adjusted_annualized_paycheck - SINGLE_BRACKET_1_THRESHOLD) * TAX_BRACKET_2_RATE) + SINGLE_BRACKET_2_BASE_TAX

    } else {
        adjusted_annualized_paycheck * TAX_BRACKET_1_RATE
    }
    
}

pub fn estimate_social_security_withholding(gross_paycheck: f32) -> f32 {
    gross_paycheck * SOCIAL_SECURITY_RATE
}

pub fn estimate_medicare_withholding(gross_paycheck: f32) -> f32 {
    gross_paycheck * MEDICARE_RATE
}




// UNIT TESTS FOR WITHHOLDINGS MODULE

#[cfg(test)] mod tests {
    use super::*;

    // Test Case 1: Low income earner in first tax bracket (10%)
    // Gross paycheck: $1,000 per paycheck (bi-weekly)
    // Annual gross: $26,000
    // Standard deduction: $16,100
    // Taxable income: $9,900
    // Expected annual tax: $9,900 * 0.10 = $990
    // Expected per-paycheck withholding: $990 / 26 = $38.08 (rounded)
    #[test]
    fn test_low_income_first_bracket() {
        let gross_paycheck = 1000.0;
        let filing_status = FilingStatus::Single;
        let elected_pretax_deductions = 0.0;
        
        let result = estimate_paycheck_federal_withholdings(
            gross_paycheck, 
            filing_status, 
            elected_pretax_deductions
        );
        
        // Expected calculation:
        // Annual gross: 1000 * 26 = 26,000
        // After standard deduction: 26,000 - 16,100 = 9,900
        // Tax: 9,900 * 0.10 = 990
        // Per paycheck: 990 / 26 = 38.076923...
        assert!((result - 38.076923).abs() < 0.01, 
            "Expected ~38.08, got {}", result);
    }

    // Test Case 2: Middle income earner crossing into third tax bracket (22%)
    // Gross paycheck: $4,500 per paycheck (bi-weekly)
    // Annual gross: $117,000
    // Standard deduction: $16,100
    // Elected pretax deductions: $5,200 (e.g., 401k)
    // Taxable income: $95,700
    // This falls in the 22% bracket (between $50,400 and $105,700)
    // Expected annual tax: $5,800 (base) + ($95,700 - $50,400) * 0.22 = $5,800 + $9,966 = $15,766
    // Expected per-paycheck withholding: $15,766 / 26 = $606.38 (rounded)
    #[test]
    fn test_middle_income_third_bracket_with_pretax() {
        let gross_paycheck = 4500.0;
        let filing_status = FilingStatus::Single;
        let elected_pretax_deductions = 5200.0;
        
        let result = estimate_paycheck_federal_withholdings(
            gross_paycheck, 
            filing_status, 
            elected_pretax_deductions
        );
        
        // Expected calculation:
        // Annual gross: 4500 * 26 = 117,000
        // After standard deduction and pretax: 117,000 - 16,100 - 5,200 = 95,700
        // Tax: 5,800 + (95,700 - 50,400) * 0.22 = 5,800 + 9,966 = 15,766
        // Per paycheck: 15,766 / 26 = 606.38461...
        assert!((result - 606.38461).abs() < 0.01, 
            "Expected ~606.38, got {}", result);
    }

    // Test Case 3: High income earner in fifth tax bracket (32%)
    // Gross paycheck: $12,000 per paycheck (bi-weekly)
    // Annual gross: $312,000
    // Standard deduction: $16,100
    // Elected pretax deductions: $23,500 (maxed out 401k)
    // Taxable income: $272,400
    // This falls in the 32% bracket (between $201,775 and $256,225... wait, 272,400 > 256,225)
    // This is actually in the 35% bracket (between $256,225 and $640,600)
    // Expected annual tax: $58,448 (base) + ($272,400 - $256,225) * 0.35 = $58,448 + $5,661.25 = $64,109.25
    // Expected per-paycheck withholding: $64,109.25 / 26 = $2,465.74 (rounded)
    #[test]
    fn test_high_income_sixth_bracket_with_max_pretax() {
        let gross_paycheck = 12000.0;
        let filing_status = FilingStatus::Single;
        let elected_pretax_deductions = 23500.0;
        
        let result = estimate_paycheck_federal_withholdings(
            gross_paycheck, 
            filing_status, 
            elected_pretax_deductions
        );
        
        // Expected calculation:
        // Annual gross: 12,000 * 26 = 312,000
        // After standard deduction and pretax: 312,000 - 16,100 - 23,500 = 272,400
        // Tax: 58,448 + (272,400 - 256,225) * 0.35 = 58,448 + 5,661.25 = 64,109.25
        // Per paycheck: 64,109.25 / 26 = 2,465.740384...
        assert!((result - 2465.740384).abs() < 0.01, 
            "Expected ~2,465.74, got {}", result);
    }

    // TESTS FOR ESTIMATE_SOCIAL_SECURITY_WITHHOLDING FUNCTION
    #[test]
    fn test_social_security_withholding() {
        let gross_paycheck = 2000.0;
        let result = estimate_social_security_withholding(gross_paycheck);
        let expected = gross_paycheck * 0.062; // 6.2%
        assert!((result - expected).abs() < 0.01, 
            "Expected {}, got {}", expected, result);
    }
    // TESTS FOR ESTIMATE_MEDICARE_WITHHOLDING FUNCTION
    #[test]
    fn test_medicare_withholding() {
        let gross_paycheck = 2000.0;
        let result = estimate_medicare_withholding(gross_paycheck);
        let expected = gross_paycheck * 0.0145; // 1.45%
        assert!((result - expected).abs() < 0.01, 
            "Expected {}, got {}", expected, result);
    }
}