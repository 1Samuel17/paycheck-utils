/// Module for handling income calculations
/// This module provides structures and functions to calculate gross income based on hourly wage or salary,
/// including considerations for overtime and paid time off.
/// It also includes utility functions to convert between hourly rates and annual salaries.

use crate::utils::{OVERTIME_MULTIPLIER, PAY_PERIOD, STANDARD_HOURS_PER_PAY_PERIOD, WEEKS_PER_YEAR};


pub enum IncomeType {
    Hourly (f32), // hourly rate
    Salary (u32) // annual salary
}
pub struct Income {
    pub income_type: IncomeType,
    pub hours_per_pay_period: Option<u32>, // only relevant for hourly income

}

impl Income {
    // per pay period
    pub fn income_per_pay_period(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {
                (salary / WEEKS_PER_YEAR * PAY_PERIOD) as f32
            },
            IncomeType::Hourly(rate) => {
                let regular_hours = 
                    if self.hours_per_pay_period.unwrap() > STANDARD_HOURS_PER_PAY_PERIOD {STANDARD_HOURS_PER_PAY_PERIOD as f32}
                    else {self.hours_per_pay_period.unwrap() as f32};
                let overtime_hours = 
                    if self.hours_per_pay_period.unwrap() > STANDARD_HOURS_PER_PAY_PERIOD {self.hours_per_pay_period.unwrap() - STANDARD_HOURS_PER_PAY_PERIOD}
                    else {0};
                (regular_hours * rate) + (overtime_hours as f32 * rate * OVERTIME_MULTIPLIER) as f32
            },
        }
    }

    // per month
    pub fn income_per_month(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {(salary / 12) as f32},
            IncomeType::Hourly(rate) => {
                let regular_hours = 
                    if self.hours_per_pay_period.unwrap() > STANDARD_HOURS_PER_PAY_PERIOD {STANDARD_HOURS_PER_PAY_PERIOD as f32}
                    else {self.hours_per_pay_period.unwrap() as f32};
                let overtime_hours = 
                    if self.hours_per_pay_period.unwrap() > STANDARD_HOURS_PER_PAY_PERIOD {self.hours_per_pay_period.unwrap() - STANDARD_HOURS_PER_PAY_PERIOD}
                    else {0};
                (regular_hours * rate) + (overtime_hours as f32 * rate * OVERTIME_MULTIPLIER) * 2 as f32
            }
        }
    }


    // per year




}


// Quick tests while developing
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_income_per_pay_period_salary() {
        let income = Income {
            income_type: IncomeType::Salary(52000),
            hours_per_pay_period: None,
        };
        assert_eq!(income.income_per_pay_period(), 2000.0);
    }
    #[test]
    fn test_income_per_pay_period_hourly_no_overtime() {
        let income = Income {
            income_type: IncomeType::Hourly(20.0),
            hours_per_pay_period: Some(80),
        };
        assert_eq!(income.income_per_pay_period(), 1600.0);
    }
    #[test]
    fn test_income_per_pay_period_hourly_with_overtime() {
        let income = Income {
            income_type: IncomeType::Hourly(20.0),
            hours_per_pay_period: Some(90),
        };
        assert_eq!(income.income_per_pay_period(), 1900.0);
    }
}
