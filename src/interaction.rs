//! This module handles all user interaction to gather the necessary information to create an employment scenario struct
//! This includes functions for displaying prompts, receiving input, and showing results. The main function in this module is `get_user_input` which orchestrates the entire process of gathering information from the user and creating an employment scenario struct.
//!
//! The `get_user_input` function first prompts the user to create an employment scenario by calling the `create_scenario` function, which gathers information about the user's hourly rate and hours worked per week. Then it prompts the user to enter their living expenses by calling the `get_expenses` function, which gathers information about various expense categories. Finally, it prompts the user to enter their deductions by calling the `get_deductions` function, which gathers information about both pre-tax and post-tax deductions. After gathering all the necessary information, it confirms the inputs with the user and then converts the inputs into an employment scenario struct using the `convert_inputs_to_struct` function.
//!
//! The `create_scenario`, `get_expenses`, and `get_deductions` functions all follow a similar pattern of prompting the user for input, validating the input, and storing it in a HashMap. The `confirm_inputs` function is used to display the gathered information back to the user for confirmation before proceeding to create the employment scenario struct. The `convert_inputs_to_struct` function takes the gathered information from the HashMaps and constructs an `EmploymentScenario` struct with the appropriate fields populated based on the user's input.

/// checks the converted value of the user input to ensure it can be parsed into the expected type (in this case, a float). If the conversion is successful, it returns true; otherwise, it returns false. This function is used in the input validation loops in the `create_scenario`, `get_expenses`, and `get_deductions` functions to ensure that the user enters valid numeric input for the various fields.
use crate::utils::check_converted_value;
use crate::{
    EmploymentScenario, Expense, Expenses, PostTaxDeduction, PostTaxDeductions, PreTaxDeduction,
    PreTaxDeductions,
};
use std::any::TypeId;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

/// main function to orchestrate user input and create employment scenario struct
/// This function will call the other functions in this module to gather information from the user and create an employment scenario struct based on that information. It will start by getting the payrate and hours worked per week.
pub fn get_user_input() -> EmploymentScenario {
    println!(
        "\n{:^100}",
        "--- Let's start by gathering some information. ---"
    );

    // get employment scenario input
    println!("\nFirst, let's create an employment scenario.\n");
    let scenario = create_scenario();

    // get expenses input
    println!(
        "\n{:^100}",
        "--- Great!, now let's create some expenses. ---"
    );
    let expenses = get_expenses();

    // get deductions input
    println!(
        "\n{:^100}",
        "--- Finally, let's create some deductions. ---"
    );
    let deductions = get_deductions();

    // create employment scenario struct using the inputs received from the user
    convert_inputs_to_struct(scenario, expenses, deductions)
}

/// create scenario input by prompting the user for their hourly rate and hours worked per week. The input is cleaned and validated to ensure it can be converted to a float before storing it in a HashMap. The keys of the HashMap are "Rate" and "Hours" and the values are the user input for those fields.
fn create_scenario() -> HashMap<String, String> {
    let mut inputs: HashMap<String, String> = HashMap::new();
    let mut input = String::new();
    let employed = ["Rate", "Hours"];

    for value in employed {
        print!("{value}: ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).unwrap_or_default();
        loop {
            if check_converted_value(&input.trim().parse::<f32>(), TypeId::of::<f32>()) {
                break;
            } else {
                print!(
                    "Please enter a valid number for {value} (examples: 25, 25.5, or 25.00) --> {value}: "
                );
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs
            .entry(value.trim().to_string())
            .or_insert(input.trim().to_string());
        input.clear();
    }

    inputs
}

/// prompt user for expenses input and return a HashMap of the inputs. Cleans the input and validates that it can be converted to a float before storing it in the HashMap. The keys of the HashMap are the expense categories and the values are the amounts entered by the user.
fn get_expenses() -> HashMap<String, String> {
    let mut inputs: HashMap<String, String> = HashMap::new();
    let mut input = String::new();
    let expense_categories = [
        "Housing",
        "Energy",
        "Water",
        "Gas",
        "Internet",
        "Phone",
        "Car Payment",
        "Car Insurance",
        "Car Gas",
        "Groceries",
    ];

    println!("\nLiving expenses can vary so enter an estimated amount per month or 0.\n");

    for exp in expense_categories {
        print!("{exp}: ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).unwrap_or_default();
        loop {
            if check_converted_value(&input.trim().parse::<f32>(), TypeId::of::<f32>()) {
                break;
            } else {
                print!(
                    "Please enter a valid number for {exp} (examples: 25, 25.5, or 25.00) --> {exp}: "
                );
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs
            .entry(exp.trim().to_string())
            .or_insert(input.trim().to_string());
        input.clear();
    }

    inputs
}

/// prompt user for deductions input and return a HashMap of the inputs. Cleans the input and validates that it can be converted to a float before storing it in the HashMap. The keys of the HashMap are the deduction categories and the values are the amounts entered by the user. This function handles both pre-tax and post-tax deductions, prompting the user separately for each type of deduction.
fn get_deductions() -> HashMap<String, String> {
    let mut inputs: HashMap<String, String> = HashMap::new();
    let mut input = String::new();
    let pretax_categories = [
        "Medical",
        "Dental",
        "Vision",
        "Traditional401K",
        "HSA",
        "FSA",
    ];
    let posttax_categories = [
        "Roth401K",
        "Voluntary Life",
        "Voluntary ADD",
        "Voluntary STD",
        "Voluntary LTD",
        "Wage Garnishment",
    ];

    println!(
        "\nDeductions come in two flavors: pre-tax and post-tax.\nLet's start with the pre-tax deductions.\n"
    );

    for pre in pretax_categories {
        print!("{pre}: ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).unwrap_or_default();
        loop {
            if check_converted_value(&input.trim().parse::<f32>(), TypeId::of::<f32>()) {
                break;
            } else {
                print!(
                    "Please enter a valid number for {pre} (examples: 25, 25.5, or 25.00) --> {pre}: "
                );
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs
            .entry(pre.trim().to_string())
            .or_insert(input.trim().to_string());
        input.clear();
    }

    println!("\nOk, now the post-tax deductions.\n");

    for post in posttax_categories {
        print!("{post}: ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).unwrap_or_default();
        loop {
            if check_converted_value(&input.trim().parse::<f32>(), TypeId::of::<f32>()) {
                break;
            } else {
                print!(
                    "Please enter a valid number for {post} (examples: 25, 25.5, or 25.00) --> {post}: "
                );
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs
            .entry(post.trim().to_string())
            .or_insert(input.trim().to_string());
        input.clear();
    }

    inputs
}

/// This function takes the three HashMaps containing the user input for the employment scenario, expenses, and deductions, and converts them into an `EmploymentScenario` struct. It parses the string values from the HashMaps into the appropriate types (e.g., f32) and constructs the `EmploymentScenario` struct with the corresponding fields populated based on the user's input.
pub fn convert_inputs_to_struct(
    sc: HashMap<String, String>,
    ex: HashMap<String, String>,
    de: HashMap<String, String>,
) -> EmploymentScenario {
    let mut scene = EmploymentScenario::default();
    scene.hourly_rate = sc["Rate"].parse::<f32>().unwrap_or_default();
    scene.hours_per_week = sc["Hours"].parse::<f32>().unwrap_or_default();
    scene.expenses = Expenses::new(vec![
        Expense::Housing(ex["Housing"].parse::<f32>().ok()),
        Expense::Energy(ex["Energy"].parse::<f32>().ok()),
        Expense::Water(ex["Water"].parse::<f32>().ok()),
        Expense::Gas(ex["Gas"].parse::<f32>().ok()),
        Expense::Internet(ex["Internet"].parse::<f32>().ok()),
        Expense::Phone(ex["Phone"].parse::<f32>().ok()),
        Expense::Vehicle(ex["Car Payment"].parse::<f32>().ok()),
        Expense::VehicleInsurance(ex["Car Insurance"].parse::<f32>().ok()),
        Expense::VehicleGas(ex["Car Gas"].parse::<f32>().ok()),
        Expense::Groceries(ex["Groceries"].parse::<f32>().ok()),
    ]);
    scene.pretax_deductions = PreTaxDeductions::new(vec![
        PreTaxDeduction::Medical(de["Medical"].parse::<f32>().ok()),
        PreTaxDeduction::Dental(de["Dental"].parse::<f32>().ok()),
        PreTaxDeduction::Vision(de["Vision"].parse::<f32>().ok()),
        PreTaxDeduction::Traditional401K(de["Traditional401K"].parse::<f32>().ok()),
        PreTaxDeduction::HSA(de["HSA"].parse::<f32>().ok()),
        PreTaxDeduction::FSA(de["FSA"].parse::<f32>().ok()),
    ]);
    scene.posttax_deductions = PostTaxDeductions::new(vec![
        PostTaxDeduction::Roth401K(de["Roth401K"].parse::<f32>().ok()),
        PostTaxDeduction::VoluntaryLife(de["Voluntary Life"].parse::<f32>().ok()),
        PostTaxDeduction::VoluntaryADD(de["Voluntary ADD"].parse::<f32>().ok()),
        PostTaxDeduction::VoluntarySTD(de["Voluntary STD"].parse::<f32>().ok()),
        PostTaxDeduction::VoluntaryLTD(de["Voluntary LTD"].parse::<f32>().ok()),
        PostTaxDeduction::WageGarnishment(de["Wage Garnishment"].parse::<f32>().ok()),
    ]);

    scene
}
