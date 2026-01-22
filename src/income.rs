/// Module for handling income calculations
/// This module provides structures and functions to calculate gross income based on hourly wage or salary,
/// including considerations for overtime and paid time off.
/// It also includes utility functions to convert between hourly rates and annual salaries.

use crate::utils::*;


pub enum IncomeType {
    Hourly (f32), // hourly rate
    Salary (f32) // annual salary
}
pub struct Income {
    pub income_type: IncomeType,
    pub hours_per_week: Option<f32>, // only relevant for hourly income
}

impl Income {
    // per week
    pub fn income_per_week(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {
                Income::round_2_decimals(salary / WEEKS_PER_YEAR)
            },
            IncomeType::Hourly(rate) => {
                let regular_hours = self.determine_regular_hours();
                let overtime_hours = self.determine_overtime_hours();
                let income = self.determine_income(regular_hours, overtime_hours, rate);
                Income::round_2_decimals(income)
            },
        }
    }
    // per pay period
    pub fn income_per_pay_period(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {
                Income::round_2_decimals((salary / WEEKS_PER_YEAR) * PAY_PERIOD)
            },
            IncomeType::Hourly(rate) => {
                let regular_hours = self.determine_regular_hours();
                let overtime_hours = self.determine_overtime_hours();
                let income = self.determine_income(regular_hours, overtime_hours, rate) * PAY_PERIOD;
                Income::round_2_decimals(income)
            },
        }
    }

    // per month
    pub fn income_per_month(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {Income::round_2_decimals(salary / MONTHS_PER_YEAR)},
            IncomeType::Hourly(rate) => {
                let regular_hours = self.determine_regular_hours();
                let overtime_hours = self.determine_overtime_hours();
                let income = self.determine_income(regular_hours, overtime_hours, rate) * WEEKS_PER_YEAR / MONTHS_PER_YEAR;
                Income::round_2_decimals(income)
            }
        }
    }
    // per year
    pub fn income_per_year(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {salary},
            IncomeType::Hourly(rate) => {self.income_per_month() * MONTHS_PER_YEAR }
        }
    }
    // helper functions
    pub fn determine_regular_hours(&self) -> f32 {
        if self.hours_per_week.unwrap() > STANDARD_HOURS_PER_WEEK {STANDARD_HOURS_PER_WEEK as f32}
        else {self.hours_per_week.unwrap() as f32}
    }
    pub fn determine_overtime_hours(&self) -> f32 {
        if self.hours_per_week.unwrap() > STANDARD_HOURS_PER_WEEK {(self.hours_per_week.unwrap() - STANDARD_HOURS_PER_WEEK) as f32}
        else {0.0}
    }
    pub fn determine_income(&self, regular_hours: f32, overtime_hours: f32, rate: f32) -> f32 {
        (regular_hours * rate) + (overtime_hours as f32 * rate * OVERTIME_MULTIPLIER) as f32
    }

    pub fn round_2_decimals(value: f32) -> f32 {
        (value * 100.0).round() / 100.0
    }
}


// Quick tests while developing (still need to create more test cases to be more thorough)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_income_hourly() {
        let income = Income {
            income_type: IncomeType::Hourly(20.0),
            hours_per_week: Some(45.0),
        };
        assert_eq!(income.income_per_week(), 950.00);
        assert_eq!(income.income_per_pay_period(), 1900.00);
        assert_eq!(income.income_per_month(), 4116.67);
        assert_eq!(income.income_per_year(), 49400.04);
    }
    #[test]
    fn test_income_salary() {
        let income = Income {
            income_type: IncomeType::Salary(60000.00),
            hours_per_week: None,
        };
        assert_eq!(income.income_per_week(), 1153.85);
        assert_eq!(income.income_per_pay_period(), 2307.69);
        assert_eq!(income.income_per_month(), 5000.00);
        assert_eq!(income.income_per_year(), 60000.00);
    }

}
