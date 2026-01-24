/// Utility constants and enums for payroll calculations.


// time related constants
pub const WEEKS_PER_YEAR: f32 = 52.0; // potential working weeks in a year
pub const WEEKS_PER_MONTH: f32 = 4.33; // average weeks in a month
pub const PAY_PERIODS_PER_YEAR: f32 = 26.0; // bi-weekly pay periods in a year
pub const MONTHS_PER_YEAR: f32 = 12.0; // months in a year
pub const STANDARD_HOURS_PER_WEEK: f32 = 40.0; // standard full-time hours per week
pub const OVERTIME_MULTIPLIER: f32 = 1.5; // time and a half
pub const PAY_PERIOD: f32 = 2.0; // 2 week pay periods
pub const PAY_PERIODS_PER_MONTH: f32 = 2.0; // pay periods in a month;
// possibly incorporate this in future versions to account for no overtime during paid time off
// pub const PAID_TIME_OFF_WEEKS_PER_YEAR: f32 = 3.0; // overtime not possible during PTO


// payroll withholding constants
pub const SOCIAL_SECURITY: f32 = 6.2; // percentage
pub const MEDICARE: f32 = 1.45; // percentage

// tax brackets as of 2026 for federal withholdings
pub const BRACKET_1: f32 = 0.10; // 10%
pub const BRACKET_2: f32 = 0.12; // 12%
pub const BRACKET_3: f32 = 0.22; // 22%
pub const BRACKET_4: f32 = 0.24; // 24%
pub const BRACKET_5: f32 = 0.32; // 32%
pub const BRACKET_6: f32 = 0.35; // 35%
pub const BRACKET_7: f32 = 0.37; // 37%

// tax brackets thresholds for single filers
pub const SINGLE_BRACKET_1_THRESHOLD: f32 = 12400.00;
pub const SINGLE_BRACKET_2_THRESHOLD: f32 = 50400.00;
pub const SINGLE_BRACKET_3_THRESHOLD: f32 = 105700.00;
pub const SINGLE_BRACKET_4_THRESHOLD: f32 = 201775.00;
pub const SINGLE_BRACKET_5_THRESHOLD: f32 = 256225.00;
pub const SINGLE_BRACKET_6_THRESHOLD: f32 = 640600.00;
pub const SINGLE_BRACKET_7_THRESHOLD: f32 = f32::MAX;

// tax brackets thresholds for married filing jointly filers
pub const MARRIED_JOINTLY_BRACKET_1_THRESHOLD: f32 = 24800.00;
pub const MARRIED_JOINTLY_BRACKET_2_THRESHOLD: f32 = 100800.00;
pub const MARRIED_JOINTLY_BRACKET_3_THRESHOLD: f32 = 211400.00;
pub const MARRIED_JOINTLY_BRACKET_4_THRESHOLD: f32 = 403550.00;
pub const MARRIED_JOINTLY_BRACKET_5_THRESHOLD: f32 = 512450.00;
pub const MARRIED_JOINTLY_BRACKET_6_THRESHOLD: f32 = 768700.00;
pub const MARRIED_JOINTLY_BRACKET_7_THRESHOLD: f32 = f32::MAX;

// tax brackets thresholds for married filing separately filers
pub const MARRIED_SEPARATELY_BRACKET_1_THRESHOLD: f32 = 12400.00;
pub const MARRIED_SEPARATELY_BRACKET_2_THRESHOLD: f32 = 50400.00;
pub const MARRIED_SEPARATELY_BRACKET_3_THRESHOLD: f32 = 105700.00;
pub const MARRIED_SEPARATELY_BRACKET_4_THRESHOLD: f32 = 201775.00;
pub const MARRIED_SEPARATELY_BRACKET_5_THRESHOLD: f32 = 256225.00;
pub const MARRIED_SEPARATELY_BRACKET_6_THRESHOLD: f32 = 384350.00;
pub const MARRIED_SEPARATELY_BRACKET_7_THRESHOLD: f32 = f32::MAX;

// tax brackets thresholds for head of household filers
pub const HEAD_OF_HOUSEHOLD_BRACKET_1_THRESHOLD: f32 = 17700.00;
pub const HEAD_OF_HOUSEHOLD_BRACKET_2_THRESHOLD: f32 = 67450.00;
pub const HEAD_OF_HOUSEHOLD_BRACKET_3_THRESHOLD: f32 = 105700.00;
pub const HEAD_OF_HOUSEHOLD_BRACKET_4_THRESHOLD: f32 = 201750.00;
pub const HEAD_OF_HOUSEHOLD_BRACKET_5_THRESHOLD: f32 = 256200.00;
pub const HEAD_OF_HOUSEHOLD_BRACKET_6_THRESHOLD: f32 = 640600.00;
pub const HEAD_OF_HOUSEHOLD_BRACKET_7_THRESHOLD: f32 = f32::MAX;


// filing statuses for federal withholdings
pub enum FilingStatus {
    Single,
    MarriedFilingJointly,
    MarriedFilingSeparately,
    HeadOfHousehold,
}