/// Module for handling paycheck income calculations
/// When considering hourly income, this module attempts to make calculations from a weekly perspective first, then expands to pay period, monthly, and yearly calculations to synthesize how an employee thinks, views, and plans their income.
/// For salaried income, calculations are more straightforward as they can be directly divided into pay periods, months, and years.

use crate::utils::*;


pub enum IncomeType {
    Hourly (f32), // hourly rate
    Salary (f32) // annual salary
}
pub struct Income {
    pub income_type: IncomeType,
    pub working_hours: Option<f32>, // only relevant for hourly income
}

impl Income {

    // per week
    pub fn gross_income_per_week(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {
                Income::round_2_decimals(salary / WEEKS_PER_YEAR)
            },
            IncomeType::Hourly(rate) => {
                self.determine_gross_income(rate)
            },
        }
    }

    // per pay period
    pub fn gross_income_per_pay_period(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {
                Income::round_2_decimals(salary / PAY_PERIODS_PER_YEAR)
            },
            IncomeType::Hourly(rate) => {
                self.determine_gross_income(rate) * PAY_PERIOD
            },
        }
    }

    // per month
    pub fn gross_income_per_month(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {Income::round_2_decimals(salary / MONTHS_PER_YEAR)},
            IncomeType::Hourly(rate) => {
                self.determine_gross_income(rate) * PAY_PERIODS_PER_MONTH
            }
        }
    }

    // per year
    pub fn gross_income_per_year(&self) -> f32 {
        match self.income_type {
            IncomeType::Salary(salary) => {salary},
            IncomeType::Hourly(rate) => {
                self.determine_gross_income(rate) * WEEKS_PER_YEAR}
        }
    }

    // helper functions
    pub fn round_2_decimals(value: f32) -> f32 {
        (value * 100.0).round() / 100.0
    }

    pub fn determine_gross_income(&self, rate: f32) -> f32 {

        let regular_hours = 
            if self.working_hours.unwrap() > STANDARD_HOURS_PER_WEEK {
                STANDARD_HOURS_PER_WEEK
            }
            else {self.working_hours.unwrap()};

        let overtime_hours = 
            if self.working_hours.unwrap() > STANDARD_HOURS_PER_WEEK {
                self.working_hours.unwrap() - STANDARD_HOURS_PER_WEEK
            }
            else {0.0};
        
        let gross_income = (regular_hours * rate) + (overtime_hours * rate * OVERTIME_MULTIPLIER);

        Income::round_2_decimals(gross_income)
    }
}


// Quick tests while developing (still need to create more test cases to be more thorough)