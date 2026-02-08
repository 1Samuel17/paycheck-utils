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

            get_user_input();
            
            // Perform paycheck calculation based on confirmed inputs

            // Display the calculated paycheck details

            // Restart or exit based on user choice 

        }
    }

    Ok(())
}