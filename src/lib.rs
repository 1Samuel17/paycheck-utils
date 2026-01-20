//! This is a library that contains utility functions for calculating net income, necessary hours, elected deductions, and standard tax withholding given set of expenses and a hypothetical hourly wage or salary scenario. The idea is pretty much like the "Sample Paycheck" tool found in the [Paycom](https://www.paycom.com/software/employee-self-service/) employee portal, but aimed at having more functionality and customization.
//! 
//! The library is structured into several modules:
//! - `taxes`: Contains functions and data structures related to tax calculations, including federal, state, and local taxes.
//! - `deductions`: Contains functions and data structures for calculating various deductions such as health insurance, retirement contributions, and other elected deductions.
//! - `income`: Contains functions for calculating gross and net income based on hourly wage or salary.
//! - `hours`: Contains functions for calculating necessary work hours to meet financial goals after taxes and deductions.
//! - `utils`: Contains utility functions that support the main calculations, such as formatting and validation.
//! Each module is designed to be as independent as possible, allowing users to utilize only the parts they need for their specific use cases.
//! 
//! 

mod taxes;
mod deductions;
mod income;
mod hours;
mod utils;
mod expenses;