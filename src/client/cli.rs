use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Accepts an array of Clock objects and a \
max weight and returns best value for weight combinations for given clocks", long_about = None)]
#[command(name = "Knapsack Solver")]
#[command(version = "1.0")]
pub struct Cli {
  #[command(subcommand)]
  pub mode: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Cli(CliArgs),
  Tui,
}

#[derive(Args, Debug)]
pub struct CliArgs {
  #[arg(long, short_alias = 'f', value_name = "FILE")]
  pub clocks_file: PathBuf,

  #[arg(long, short, value_parser = clap::value_parser!(f32), value_name = "FLOAT")]
  pub weight: f32,
}
