/// Module for handling income calculations
/// This module provides structures and functions to calculate gross income based on hourly wage or salary,
/// including considerations for overtime and paid time off.
/// It also includes utility functions to convert between hourly rates and annual salaries.

use crate::utils::{WEEKS_PER_YEAR, STANDARD_HOURS_PER_WEEK, OVERTIME_MULTIPLIER, PAID_TIME_OFF_WEEKS_PER_YEAR};


pub enum IncomeType {
    Hourly (u32), // hourly rate
    Salary (u32) // annual salary
}
pub struct Income {
    pub income_type: IncomeType,
    pub hours_per_week: Option<u32>, // only relevant for hourly income
    pub overtime_hours_per_week: Option<u32>, // only relevant for hourly income
}

impl Income {
    pub fn gross_annual_income(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => salary as f32,
            IncomeType::Hourly(rate) => {
                let hours = self.hours_per_week.unwrap_or(STANDARD_HOURS_PER_WEEK);
                let overtime_hours = self.overtime_hours_per_week.unwrap_or(0);
                let regular_income = rate as f32 * hours as f32 * WEEKS_PER_YEAR as f32;
                let overtime_income = rate as f32 * OVERTIME_MULTIPLIER * overtime_hours as f32 * (WEEKS_PER_YEAR - PAID_TIME_OFF_WEEKS_PER_YEAR) as f32;
                regular_income + overtime_income
            }
        }
    }

    pub fn salary_to_hourly(salary: u32) -> f32 {
        salary as f32 / (WEEKS_PER_YEAR as f32 * STANDARD_HOURS_PER_WEEK as f32)
    }

    pub fn hourly_to_salary(hourly_rate: u32) -> f32 {
        hourly_rate as f32 * WEEKS_PER_YEAR as f32 * STANDARD_HOURS_PER_WEEK as f32
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gross_annual_income_hourly() {
        let income = Income {
            income_type: IncomeType::Hourly(20),
            hours_per_week: Some(40),
            overtime_hours_per_week: Some(5),
        };
        let gross_income = income.gross_annual_income();
        assert_eq!(gross_income, 20.0 * 40.0 * 52.0 + 20.0 * 1.5 * 5.0 * 49.0);
    }

    #[test]
    fn test_gross_annual_income_salary() {
        let income = Income {
            income_type: IncomeType::Salary(60000),
            hours_per_week: None,
            overtime_hours_per_week: None,
        };
        let gross_income = income.gross_annual_income();
        assert_eq!(gross_income, 60000.0);
    }

    #[test]
    fn test_salary_to_hourly() {
        let hourly_rate = Income::salary_to_hourly(52000);
        assert_eq!(hourly_rate, 25.0);
    }

    #[test]
    fn test_hourly_to_salary() {
        let salary = Income::hourly_to_salary(25);
        assert_eq!(salary, 52000.0);
    }

}