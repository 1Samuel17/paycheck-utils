use anyhow::Result;
use clap::{Parser, Subcommand};
use paycheck_utils::interaction::*;

#[derive(Parser, Debug)]
#[command(name = "check-paycheck")]
#[command(
    author, 
    version = "0.1.0", 
    about = "A cli tool for estimating paycheck net income and withholdings in order to compare against a given set of living expenses", 
    long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// start a dialogue to input employment scenario, deductions, and expenses
    Start,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => { 
            
            // Start the interactive dialogue to receive user input for employment scenario, deductions, and expenses
            println!("\n{:-^100}"," CHECK-PAYCHECK CLI TOOL: ");

            println!("\n{:^100}","A CLI tool for estimating paycheck net income and withholdings");
            println!("{:^100}", "in order to compare against a given set of living expenses.");
            println!("\n{:-^100}","-");

            // create a scenario from user inputs
            let scenario = get_user_input();
            
            // Perform paycheck calculation based on confirmed inputs
            let net_paycheck = scenario.calculate_net_paycheck();
            let comparison = scenario.compare_monthly_expenses_to_monthly_income();

            // Display the calculated paycheck details

            println!("\n{:^100}", "--- Paycheck Calculation Results ---");
            println!("\nWeekly Net Paycheck: ${:.2}", net_paycheck);
            println!("\nMonthly Income vs Expenses -> Monthly Net Income: {:?}, Total Monthly Expenses: {:?}, Difference: {:?}\n", comparison.0, comparison.1, comparison.2);

            // Restart or exit based on user choice (future implementation)

        }
    }

    Ok(())
}