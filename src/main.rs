use fs::read_to_string;
use std::fs;

use clap::Parser;
use rust_decimal::Decimal;
use serde_json::from_str;

use knapsack_problem::client::cli::{Cli, Commands};
use knapsack_problem::problem::Problem;
use knapsack_problem::{Clock, GenericResult};

fn main() -> GenericResult<()> {
  let cli = Cli::parse();

  match &cli.mode {
    Commands::Cli(args) => {
      let buffer = read_to_string(&args.clocks_file)?;

      let clocks: Vec<Clock> = from_str(&buffer)?;
      let weight = Decimal::from_f32_retain(args.weight)
        .filter(|decimal| decimal.is_sign_positive())
        .ok_or_else(|| format!("{} must be a positive FLOAT number", &args.weight))?;
      let problem = Problem::new(&clocks, weight);
      if let Some(result) = problem.get_best_knapsack() {
        println!(
          "The best Knapsack combination for the provided input is {:?}",
          result
        );
      }
    }
    Commands::Tui => {
      panic!("'Tui' is not implemented yet")
    }
  }
  Ok(())
}
