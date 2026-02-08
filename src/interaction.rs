/// module for user interaction
/// handles input and output with the user
/// includes functions for displaying prompts, receiving input, and showing results
use std::io::prelude::*;
use std::io;
use std::collections::HashMap;
use crate::utils::check_converted_value;
use std::any::TypeId;


pub fn get_user_input() {

    println!("\n{:^100}", "--- Let's start by gathering some information. ---");

    // get employment scenario input
    println!("\nFirst, let's create an employment scenario.\n");
    let scenario = create_scenario();

    //get expenses input
    println!("\n{:^100}", "--- Great!, now let's create some expenses. ---");
    let expenses = get_expenses();

    // //get deductions input
    println!("\n{:^100}", "--- Finally, let's create some deductions. ---");
    let deductions = get_deductions();

    // // confirm inputs with user
    println!("\n{:^100}", "--- Great! Let's make sure everything looks correct. ---");
    confirm_inputs(scenario, expenses, deductions);
}

fn create_scenario() -> HashMap<String, String> {
    // create holding variables
    let mut inputs: HashMap<String, String> = HashMap::new();
    let mut input = String::new();
    let employed = [
        "Rate",
        "Hours",
    ];

    for value in employed {
        print!("{value}: ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).unwrap_or_default();
        loop {
            if check_converted_value(&input.trim().parse::<f32>(), TypeId::of::<f32>()) {
                break;
            } else {
                print!("Please enter a valid number for {value} (examples: 25, 25.5, or 25.00) --> {value}: ");
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs.entry(value.trim().to_string()).or_insert(input.trim().to_string());
        input.clear();
    }

    inputs

}

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
                print!("Please enter a valid number for {exp} (examples: 25, 25.5, or 25.00) --> {exp}: ");
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs.entry(exp.trim().to_string()).or_insert(input.trim().to_string());
        input.clear();
    }

    inputs

}

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

    println!("\nDeductions come in two flavors: pre-tax and post-tax.\nLet's start with the pre-tax deductions.\n");

    for pre in pretax_categories {
        print!("{pre}: ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).unwrap_or_default();
        loop {
            if check_converted_value(&input.trim().parse::<f32>(), TypeId::of::<f32>()) {
                break;
            } else {
                print!("Please enter a valid number for {pre} (examples: 25, 25.5, or 25.00) --> {pre}: ");
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs.entry(pre.trim().to_string()).or_insert(input.trim().to_string());
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
                print!("Please enter a valid number for {post} (examples: 25, 25.5, or 25.00) --> {post}: ");
                input.clear();
                io::stdout().flush().unwrap_or_default();
                io::stdin().read_line(&mut input).unwrap_or_default();
            }
        }
        inputs.entry(post.trim().to_string()).or_insert(input.trim().to_string());
        input.clear();
    }

    inputs

}

fn confirm_inputs(
    scenario: HashMap<String, String>,
    expenses: HashMap<String, String>,
    deductions: HashMap<String, String>,
) {
    let mut input = String::new();

    println!("\nHere is the scenario we just created:\n");

    println!("EMPLOYMENT SCENARIO:\n");

    for (k, v) in scenario.iter() {
        println!("{}: {}", k, v);
    }

    println!("\nDoes this look good (y/n)?: {}", input);
    io::stdin().read_line(&mut input.trim().to_string()).unwrap_or_default();
    input.clear();

    println!("\nEXPENSES:\n");

    for (k,v) in expenses.iter() {
        println!("{}: {}", k, v);
    }

    println!("\nDoes this look good (y/n)?: {}", input);
    io::stdin().read_line(&mut input.trim().to_string()).unwrap_or_default();
    input.clear();

    println!("\nDEDUCTIONS:\n");

    for (k,v) in deductions.iter() {
        println!("{}: {}", k, v);
    }

    println!("\nDoes this look good (y/n)?: {}", input);
    io::stdin().read_line(&mut input.trim().to_string()).unwrap_or_default();
    input.clear();

    println!("\n{:^100}", "--- Great! Let's run the numbers! ---");

}