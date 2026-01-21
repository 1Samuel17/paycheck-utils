/// Utility constants for payroll calculations.

pub const WEEKS_PER_YEAR: u32 = 52; // potential working weeks in a year
pub const STANDARD_HOURS_PER_PAY_PERIOD: u32 = 80; // standard full-time hours
pub const OVERTIME_MULTIPLIER: f32 = 1.5; // time and a half
pub const PAID_TIME_OFF_WEEKS_PER_YEAR: u32 = 3; // overtime not possible during PTO
pub const PAY_PERIOD: u32 = 2; // 2 week pay periods