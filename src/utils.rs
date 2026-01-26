/// Utility constants and enums for payroll calculations.


// time related constants
pub const PAY_PERIOD: f32 = 2.0; // 2 week pay periods
pub const PAY_PERIODS_PER_YEAR: f32 = 26.0; // bi-weekly pay periods in a year
pub const STANDARD_HOURS_PER_WEEK: f32 = 40.0; // standard full-time hours per week
pub const OVERTIME_MULTIPLIER: f32 = 1.5; // time and a half
// pub const PAID_TIME_OFF_WEEKS_PER_YEAR: f32 = 3.0; // possible future integration of overtime not possible during PTO

// 2026 filing status with standard deduction as associated value
pub enum FilingStatus {
    Single, 
    MarriedFilingJointly, // for future implementation
    MarriedFilingSeparate, // for future implementation
    HeadOfHousehold, // for future implementation
}

// 2026 standard deductions (irs.gov)
pub const SINGLE_DEDUCTION: f32 = 16100.00;
// pub const HEAD_OF_HOUSEHOLD_DEDUCTION: f32 = 24150.00; // for future implementation
// pub const MARRIED_FILING_JOINTLY_DEDUCTION: f32 = 32200.00; // for future implementation
// pub const MARRIED_FILING_SEPERATE_DEDUCTION: f32 = 16100.00; // for future implementation

// 2026 payroll withholding constants
pub const SOCIAL_SECURITY_RATE: f32 = 0.062; // percentage
pub const MEDICARE_RATE: f32 = 0.0145; // percentage

// 2026 tax brackets for federal withholdings (irs.gov)
pub const TAX_BRACKET_1_RATE: f32 = 0.10; // 10%
pub const TAX_BRACKET_2_RATE: f32 = 0.12; // 12%
pub const TAX_BRACKET_3_RATE: f32 = 0.22; // 22%
pub const TAX_BRACKET_4_RATE: f32 = 0.24; // 24%
pub const TAX_BRACKET_5_RATE: f32 = 0.32; // 32%
pub const TAX_BRACKET_6_RATE: f32 = 0.35; // 35%
pub const TAX_BRACKET_7_RATE: f32 = 0.37; // 37%

// 2026 tax brackets thresholds for single filers (irs.gov)
pub const SINGLE_BRACKET_1_THRESHOLD: f32 = 12400.00;
pub const SINGLE_BRACKET_2_THRESHOLD: f32 = 50400.00;
pub const SINGLE_BRACKET_3_THRESHOLD: f32 = 105700.00;
pub const SINGLE_BRACKET_4_THRESHOLD: f32 = 201775.00;
pub const SINGLE_BRACKET_5_THRESHOLD: f32 = 256225.00;
pub const SINGLE_BRACKET_6_THRESHOLD: f32 = 640600.00;

// 2026 tax brackets base tax amounts for single filers (Copilot generated)
pub const SINGLE_BRACKET_2_BASE_TAX: f32 = 1240.00;
pub const SINGLE_BRACKET_3_BASE_TAX: f32 = 5800.00;
pub const SINGLE_BRACKET_4_BASE_TAX: f32 = 17996.00;
pub const SINGLE_BRACKET_5_BASE_TAX: f32 = 41024.00;
pub const SINGLE_BRACKET_6_BASE_TAX: f32 = 58448.00;
pub const SINGLE_BRACKET_7_BASE_TAX: f32 = 192979.25;



// FUTURE IMPLEMENTATION OF OTHER FILING STATUS TAX BRACKETS

// // 2026 tax brackets thresholds for married filing jointly filers
// pub const MARRIED_JOINTLY_BRACKET_1_THRESHOLD: f32 = 24800.00;
// pub const MARRIED_JOINTLY_BRACKET_2_THRESHOLD: f32 = 100800.00;
// pub const MARRIED_JOINTLY_BRACKET_3_THRESHOLD: f32 = 211400.00;
// pub const MARRIED_JOINTLY_BRACKET_4_THRESHOLD: f32 = 403550.00;
// pub const MARRIED_JOINTLY_BRACKET_5_THRESHOLD: f32 = 512450.00;
// pub const MARRIED_JOINTLY_BRACKET_6_THRESHOLD: f32 = 768700.00;
// pub const MARRIED_JOINTLY_BRACKET_7_THRESHOLD: f32 = f32::MAX;

// // 2026 tax brackets thresholds for married filing separately filers
// pub const MARRIED_SEPARATELY_BRACKET_1_THRESHOLD: f32 = 12400.00;
// pub const MARRIED_SEPARATELY_BRACKET_2_THRESHOLD: f32 = 50400.00;
// pub const MARRIED_SEPARATELY_BRACKET_3_THRESHOLD: f32 = 105700.00;
// pub const MARRIED_SEPARATELY_BRACKET_4_THRESHOLD: f32 = 201775.00;
// pub const MARRIED_SEPARATELY_BRACKET_5_THRESHOLD: f32 = 256225.00;
// pub const MARRIED_SEPARATELY_BRACKET_6_THRESHOLD: f32 = 384350.00;
// pub const MARRIED_SEPARATELY_BRACKET_7_THRESHOLD: f32 = f32::MAX;

// // 2026 tax brackets thresholds for head of household filers
// pub const HEAD_OF_HOUSEHOLD_BRACKET_1_THRESHOLD: f32 = 17700.00;
// pub const HEAD_OF_HOUSEHOLD_BRACKET_2_THRESHOLD: f32 = 67450.00;
// pub const HEAD_OF_HOUSEHOLD_BRACKET_3_THRESHOLD: f32 = 105700.00;
// pub const HEAD_OF_HOUSEHOLD_BRACKET_4_THRESHOLD: f32 = 201750.00;
// pub const HEAD_OF_HOUSEHOLD_BRACKET_5_THRESHOLD: f32 = 256200.00;
// pub const HEAD_OF_HOUSEHOLD_BRACKET_6_THRESHOLD: f32 = 640600.00;
// pub const HEAD_OF_HOUSEHOLD_BRACKET_7_THRESHOLD: f32 = f32::MAX;
