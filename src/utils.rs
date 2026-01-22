/// Utility constants for payroll calculations.

pub const WEEKS_PER_YEAR: f32 = 52.0; // potential working weeks in a year
pub const MONTHS_PER_YEAR: f32 = 12.0; // months in a year
pub const STANDARD_HOURS_PER_WEEK: f32 = 40.0; // standard full-time hours per week
pub const OVERTIME_MULTIPLIER: f32 = 1.5; // time and a half
pub const PAID_TIME_OFF_WEEKS_PER_YEAR: f32 = 3.0; // overtime not possible during PTO
pub const PAY_PERIOD: f32 = 2.0; // 2 week pay periods