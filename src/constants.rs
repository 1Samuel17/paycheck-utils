//! Module containing utility constants and enums for payroll calculations.
//! Tax related constants are based on IRS guidelines for the year 2026.
//! Focus is given to single filer status, with future implementation plans for other filing statuses.

/// 2 week pay periods
pub const PAY_PERIOD: f32 = 2.0;

/// 26 bi-weekly pay periods in a year
pub const PAY_PERIODS_PER_YEAR: f32 = 26.0;

/// standard 40 hour full-time hours per week
pub const STANDARD_HOURS_PER_WEEK: f32 = 40.0;

/// time and a half
pub const OVERTIME_MULTIPLIER: f32 = 1.5;
// pub const PAID_TIME_OFF_WEEKS_PER_YEAR: f32 = 3.0; // possible future integration of overtime not possible during PTO

/// 2026 filing statuses
#[derive(Debug, Clone, Copy)]
pub enum FilingStatus {
    Single,
    MarriedFilingJointly,  // for future implementation
    MarriedFilingSeparate, // for future implementation
    HeadOfHousehold,       // for future implementation
}

/// 2026 standard deduction for single filer: $16,100 (source: irs.gov)
pub const SINGLE_DEDUCTION: f32 = 16100.00;

/// 2026 standard deduction for head of household filer: $24,150 (source: irs.gov)
pub const HEAD_OF_HOUSEHOLD_DEDUCTION: f32 = 24150.00; // for future implementation

/// 2026 standard deduction for married filing jointly filer: $32,200 (source: irs.gov)
pub const MARRIED_FILING_JOINTLY_DEDUCTION: f32 = 32200.00; // for future implementation

/// 2026 standard deduction for married filing separately filer: $16,100 (source: irs.gov)
pub const MARRIED_FILING_SEPERATE_DEDUCTION: f32 = 16100.00; // for future implementation

/// 2026 Social Security tax rate: 6.2% (source: irs.gov)
pub const SOCIAL_SECURITY_RATE: f32 = 0.062;

/// 2026 Medicare tax rate: 1.45% (source: irs.gov)
pub const MEDICARE_RATE: f32 = 0.0145;

/// 2026 tax bracket 1 rate: 10% (source: irs.gov)
pub const TAX_BRACKET_1_RATE: f32 = 0.10; // 10%

/// 2026 tax bracket 2 rate: 12% (source: irs.gov)
pub const TAX_BRACKET_2_RATE: f32 = 0.12; // 12%

/// 2026 tax bracket 3 rate: 22% (source: irs.gov)
pub const TAX_BRACKET_3_RATE: f32 = 0.22; // 22%

/// 2026 tax bracket 4 rate: 24% (source: irs.gov)
pub const TAX_BRACKET_4_RATE: f32 = 0.24; // 24%

/// 2026 tax bracket 5 rate: 32% (source: irs.gov)
pub const TAX_BRACKET_5_RATE: f32 = 0.32; // 32%

/// 2026 tax bracket 6 rate: 35% (source: irs.gov)
pub const TAX_BRACKET_6_RATE: f32 = 0.35; // 35%

/// 2026 tax bracket 7 rate: 37% (source: irs.gov)
pub const TAX_BRACKET_7_RATE: f32 = 0.37; // 37%

/// 2026 single filer tax bracket 1 upper limit threshold: $12,400 (source: irs.gov)
pub const SINGLE_BRACKET_1_THRESHOLD: f32 = 12400.00;

/// 2026 single filer tax bracket 2 upper limit threshold: $50,400 (source: irs.gov)
pub const SINGLE_BRACKET_2_THRESHOLD: f32 = 50400.00;

/// 2026 single filer tax bracket 3 upper limit threshold: $105,700 (source: irs.gov)
pub const SINGLE_BRACKET_3_THRESHOLD: f32 = 105700.00;

/// 2026 single filer tax bracket 4 upper limit threshold: $201,775 (source: irs.gov)
pub const SINGLE_BRACKET_4_THRESHOLD: f32 = 201775.00;

/// 2026 single filer tax bracket 5 upper limit threshold: $256,225 (source: irs.gov)
pub const SINGLE_BRACKET_5_THRESHOLD: f32 = 256225.00;

/// 2026 single filer tax bracket 6 upper limit threshold: $640,600 (source: irs.gov)
pub const SINGLE_BRACKET_6_THRESHOLD: f32 = 640600.00;

/// 2026 single filer tax bracket 2 base tax amount: $1,240.00 (source: irs.gov)
pub const SINGLE_BRACKET_2_BASE_TAX: f32 = 1240.00;

/// 2026 single filer tax bracket 3 base tax amount: $5,800.00 (source: irs.gov)
pub const SINGLE_BRACKET_3_BASE_TAX: f32 = 5800.00;

/// 2026 single filer tax bracket 4 base tax amount: $17,996.00 (source: irs.gov)
pub const SINGLE_BRACKET_4_BASE_TAX: f32 = 17996.00;

/// 2026 single filer tax bracket 5 base tax amount: $41,024.00 (source: irs.gov)
pub const SINGLE_BRACKET_5_BASE_TAX: f32 = 41024.00;

/// 2026 single filer tax bracket 6 base tax amount: $58,448.00 (source: irs.gov)
pub const SINGLE_BRACKET_6_BASE_TAX: f32 = 58448.00;

/// 2026 single filer tax bracket 7 base tax amount: $192,979.25 (source: irs.gov)
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
