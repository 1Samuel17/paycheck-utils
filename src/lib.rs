//! This library contains utility functions for calculating paycheck withholdings and net income given a hypothetical hourly wage and weekly working hours. The idea is pretty much like the "Sample Paycheck" tool found in the [Paycom](https://www.paycom.com/software/employee-self-service/) employee portal, but aimed at having a little more functionality and customization.
//!
//! The entire library was developed with the perspective of an hourly paid employee in mind, focusing on bi-weekly paychecks as the standard pay period to simulate how employees typically view and plan their income.
//!
//! The primary question this library aims to answer is: "Given an hourly wage and number of hours worked per week, what would my net paycheck be after taxes and deductions?"
//!
//! The secondary question this library aims to answer is: "Given a total monthly expenses amount and hourly wage, how many hours would I need to work to cover my expenses with "x" amount left over after taxes and deductions?" (this will be implemented in a future version).
//!
//! The library is structured into several modules:
//! - `withholdings`: Contains functions to estimate federal tax withholdings, Social Security, and Medicare deductions.
//! - `deductions`: Defines structures and functions for handling pre-tax and post-tax deductions.
//! - `income`: Contains functions to calculate gross paycheck based on hourly wage and hours worked.
//! - `expenses`: Defines structures and functions for managing monthly expenses.
//! - `constants`: Contains tax and time related constants necessary for calculations.
//! - `interaction`: Contains functions for interacting with the user to receive input for employment scenario.
//! - `utils`: Contains utility functions for rounding and formatting output.
//!
//! A CLI will be implemented in a future version to allow a user to input their hourly wage, hours worked, filing status, deductions and expenses to see their estimated net paycheck and other relevant financial metrics.

pub mod deductions;
pub mod expenses;
pub mod income;
pub mod constants;
pub mod withholdings;
pub mod interaction;
pub mod utils;

pub use crate::deductions::*;
pub use crate::expenses::*;
pub use crate::income::*;
pub use crate::constants::*;
pub use crate::withholdings::*;
pub use crate::interaction::*;
pub use crate::utils::*;

/// Represents an employment scenario with hourly rate, hours worked per week, filing status, and deductions.
/// Possible deductions avaialable are defined in the `deductions` module.
///
/// # Example
/// ```
/// use paycheck_utils::*;
///
/// let new_job_scenario = EmploymentScenario::new(
///     30.0, // hourly rate
///     40.0, // hours per week
///     FilingStatus::Single, // filing status
///     PreTaxDeductions::new(vec![
///         PreTaxDeduction::Medical(Some(150.0)),
///         PreTaxDeduction::Dental(Some(50.0)),
///         PreTaxDeduction::Vision(Some(15.0)),
///         PreTaxDeduction::Traditional401K(Some(200.0)),
///     ]), // pre-tax deductions
///     PostTaxDeductions::new(vec![PostTaxDeduction::Roth401K(Some(100.0))]), // post-tax deductions
///     Expenses::new(vec![]) // expenses
/// );
/// ```
pub struct EmploymentScenario {
    pub hourly_rate: f32,
    pub hours_per_week: f32,
    pub filing_status: FilingStatus,
    pub pretax_deductions: PreTaxDeductions,
    pub posttax_deductions: PostTaxDeductions,
    pub expenses: Expenses,
}

impl EmploymentScenario {
    pub fn new(
        hourly_rate: f32,
        hours_per_week: f32,
        filing_status: FilingStatus,
        pretax_deductions: PreTaxDeductions,
        posttax_deductions: PostTaxDeductions,
        expenses: Expenses,
    ) -> Self {
        EmploymentScenario {
            hourly_rate,
            hours_per_week,
            filing_status,
            pretax_deductions,
            posttax_deductions,
            expenses,
        }
    }

    /// Calculates the net paycheck based on the employment scenario's parameters.
    /// The calculations consider gross income, pre-tax deductions, federal tax withholdings, Social Security, Medicare, and post-tax deductions.
    /// The IRS defined constants used to make calculations (such as tax rates, thresholds and standard deductions) are defined in the `constants` module.
    /// This IRS method and flow for calculating withholdings is based on the 2026 federal tax year guidelines and can be summarized as follows:
    ///    1. Calculate gross paycheck on hourly rate and hours worked.
    ///    2. Subtract pre-tax deductions from gross paycheck to get adjusted gross paycheck.
    ///    3. Calculate federal tax withholdings based on annualized adjusted gross paycheck and filing status.
    ///    4. Calculate Social Security and Medicare withholdings based on adjusted gross paycheck.
    ///    5. Subtract federal tax withholdings, Social Security, Medicare, and post-tax deductions from adjusted gross paycheck to get net paycheck.
    ///
    /// # Example
    /// ```
    /// use paycheck_utils::*;
    ///
    /// let pretax_deductions = PreTaxDeductions::new(vec![
    ///     PreTaxDeduction::Medical(Some(100.0)),
    ///     PreTaxDeduction::Dental(Some(50.0)),
    ///     PreTaxDeduction::Vision(Some(25.0)),
    ///     PreTaxDeduction::Traditional401K(Some(200.0)),
    ///     PreTaxDeduction::HSA(Some(150.0)),
    /// ]); // total = 525.0
    /// let posttax_deductions = PostTaxDeductions::new(vec![
    ///     PostTaxDeduction::Roth401K(Some(100.0)),
    ///     PostTaxDeduction::VoluntaryLife(Some(30.0)),
    /// ]); // total = 130.0
    /// let scenario = EmploymentScenario::new(
    ///     25.0, // hourly rate
    ///     45.0, // hours per week (bi-weekly paycheck = 90 hours [10 hours overtime])
    ///     FilingStatus::Single, // single filing status for standard deduction
    ///     pretax_deductions, // total = 525.0
    ///     posttax_deductions, // total = 130.0
    ///     Expenses::new(vec![
    ///         Expense::Housing(Some(2000.0)),
    ///         Expense::Energy(Some(300.0)),
    ///     ]), // total = 2300.0
    /// );
    /// let net_paycheck = scenario.calculate_net_paycheck();
    /// assert_eq!(net_paycheck, 1440.33);
    ///
    /// // Explanation of calculation:
    /// // 1. Gross Paycheck: (25.0 * 80) + (25.0 * 10 * 1.5) = 2000.0 + 375.0 = 2375.0
    /// // 2. Adjusted Gross Paycheck: 2375.0 - 525.0 = 1850.0  (after pre-tax deductions)
    /// // 3. Federal Withholding (annualized AGP = 1850.0 * 26 = 48100.0): Using 2026 tax brackets for Single filer:
    /// //    - 10% on first 12,400 = 12,400 * 0.10 = 1,240.0
    /// //    - 12% on amount over 12,400 up to 50,400 = (48,100.0 - 12,400.0) * 0.12 = 4,290.0
    /// //    - Total annual federal tax = 1,240.0 + 4,290.0 = 5,530.0
    /// //    - Bi-weekly federal withholding = 5,530.0 / 26 = 212.69
    /// // 4. Social Security Withholding: 1850.0 * 0.062 = 114.70
    /// // 5. Medicare Withholding: 1850.0 * 0.0145 = 26.83
    /// // 6. Post-Tax Deductions: 100.0 + 30.0 = 130.0
    /// // 7. Total Deductions: 212.69 + 114.70 + 26.83 + 130.0 = 484.22
    /// // 8. Net Paycheck: 1850.0 - 212.69 - 114.70 - 26.83 - 130.0 = 1440.33
    /// ```
    /// # Returns
    /// An `f32` representing the calculated net paycheck amount.
    ///
    /// # Panics
    /// This function does not explicitly panic, but it assumes that the input values (hourly rate, hours worked, deductions) are valid and reasonable.
    ///
    /// # Errors
    /// This function does not return errors, but invalid input values may lead to incorrect calculations.
    ///
    /// # Notes
    /// The calculations are based on the 2026 federal tax year guidelines and may need to be updated for future tax years.
    pub fn calculate_net_paycheck(&self) -> f32 {
        let mut gross_paycheck = determine_gross_paycheck(self.hourly_rate, self.hours_per_week);
        let total_pretax = self.pretax_deductions.total_pretax_deductions();
        gross_paycheck -= total_pretax;
        let federal_withholding =
            estimate_paycheck_federal_withholdings(gross_paycheck, self.filing_status);
        let social_security = estimate_social_security_withholding(gross_paycheck);
        let medicare = estimate_medicare_withholding(gross_paycheck);
        let total_posttax = self.posttax_deductions.total_posttax_deductions();

        round_2_decimals(
            gross_paycheck - federal_withholding - social_security - medicare - total_posttax,
        )
    }

    /// Compares the total monthly expenses to the calculated monthly net income.
    /// Returns a tuple containing the monthly net income, total monthly expenses, and the difference between the two.
    /// # Example
    /// ```
    /// // This example uses the same data as the `calculate_net_paycheck` example to demonstrate the comparison.
    /// 
    /// use paycheck_utils::*;
    ///
    /// let pretax_deductions = PreTaxDeductions::new(vec![
    ///     PreTaxDeduction::Medical(Some(100.0)),
    ///     PreTaxDeduction::Dental(Some(50.0)),
    ///     PreTaxDeduction::Vision(Some(25.0)),
    ///     PreTaxDeduction::Traditional401K(Some(200.0)),
    ///     PreTaxDeduction::HSA(Some(150.0)),
    /// ]); // total = 525.0
    /// let posttax_deductions = PostTaxDeductions::new(vec![
    ///     PostTaxDeduction::Roth401K(Some(100.0)),
    ///     PostTaxDeduction::VoluntaryLife(Some(30.0)),
    /// ]); // total = 130.0
    /// let expenses = Expenses::new(vec![
    ///     Expense::Housing(Some(1500.0)),
    ///     Expense::Energy(Some(200.0)),
    ///     Expense::Water(Some(50.0)),
    ///     Expense::Groceries(Some(400.0)),
    ///     Expense::Phone(Some(80.0)),
    ///     Expense::Internet(Some(60.0)),
    /// ]); // total = 2290.0
    /// let scenario = EmploymentScenario::new(
    ///     25.0, // hourly rate
    ///     45.0, // hours per week
    ///     FilingStatus::Single, // filing status
    ///     pretax_deductions,
    ///     posttax_deductions,
    ///     expenses,
    /// );
    /// let (monthly_net_income, total_monthly_expenses, difference) = scenario.compare_monthly_expenses_to_monthly_income();
    /// assert_eq!(monthly_net_income, 2880.66);
    /// assert_eq!(total_monthly_expenses, 2290.0);
    /// assert_eq!(difference, 590.66);
    /// ```
    /// # Returns
    /// A tuple containing:
    /// - `f32`: Monthly net income
    /// - `f32`: Total monthly expenses
    /// - `f32`: Difference between monthly net income and total monthly expenses
    pub fn compare_monthly_expenses_to_monthly_income(&self) -> (f32, f32, f32) {
        let monthly_net_income = self.calculate_net_paycheck() * 2.0;
        let total_monthly_expenses = self.expenses.total_monthly_expenses();
        (
            round_2_decimals(monthly_net_income), 
            round_2_decimals(total_monthly_expenses), 
            round_2_decimals(monthly_net_income - total_monthly_expenses)
        )
    }
}


// UNIT TEST FOR LIBRARY

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_net_paycheck() {
        let pretax_deductions = PreTaxDeductions::new(vec![
            PreTaxDeduction::Medical(Some(100.0)),
            PreTaxDeduction::Dental(Some(50.0)),
            PreTaxDeduction::Vision(Some(25.0)),
            PreTaxDeduction::Traditional401K(Some(200.0)),
            PreTaxDeduction::HSA(Some(150.0)),
        ]);
        let posttax_deductions = PostTaxDeductions::new(vec![
            PostTaxDeduction::Roth401K(Some(100.0)),
            PostTaxDeduction::VoluntaryLife(Some(30.0)),
        ]);
        let expenses = Expenses::new(vec![
            Expense::Housing(Some(2000.0)),
            Expense::Energy(Some(200.0)),
            Expense::Water(Some(50.0)),
            Expense::Internet(Some(60.0)),
            Expense::Phone(Some(80.0)),
            Expense::Vehicle(Some(300.0)),
            Expense::VehicleInsurance(Some(150.0)),
            Expense::VehicleGas(Some(100.0)),
            Expense::Groceries(Some(400.0)),
        ]);
        let scenario = EmploymentScenario::new(
            25.0,
            45.0,
            FilingStatus::Single,
            pretax_deductions,
            posttax_deductions,
            expenses,
        );
        let net_paycheck = scenario.calculate_net_paycheck();
        assert_eq!(net_paycheck, 1440.33);
    }

    #[test]
    fn test_compare_monthly_expenses_to_monthly_income() {
        let pretax_deductions = PreTaxDeductions::new(vec![
            PreTaxDeduction::Medical(Some(100.0)),
            PreTaxDeduction::Dental(Some(50.0)),
            PreTaxDeduction::Vision(Some(25.0)),
            PreTaxDeduction::Traditional401K(Some(200.0)),
            PreTaxDeduction::HSA(Some(150.0)),
        ]);
        let posttax_deductions = PostTaxDeductions::new(vec![
            PostTaxDeduction::Roth401K(Some(100.0)),
            PostTaxDeduction::VoluntaryLife(Some(30.0)),
        ]);
        let expenses = Expenses::new(vec![
            Expense::Housing(Some(1500.0)),
            Expense::Energy(Some(200.0)),
            Expense::Water(Some(50.0)),
            Expense::Groceries(Some(400.0)),
            Expense::Phone(Some(80.0)),
            Expense::Internet(Some(60.0)),
        ]);
        let scenario = EmploymentScenario::new(
            25.0,
            45.0,
            FilingStatus::Single,
            pretax_deductions,
            posttax_deductions,
            expenses,
        );
        let (monthly_net_income, total_monthly_expenses, difference) = scenario.compare_monthly_expenses_to_monthly_income();
        assert_eq!(monthly_net_income, 2880.66);
        assert_eq!(total_monthly_expenses, 2290.0);
        assert_eq!(difference, 590.66);
    }
}
