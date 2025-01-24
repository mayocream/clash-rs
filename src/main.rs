use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'f', long = "config", value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Validate,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Validate) => {
            println!("Validating config file: {:?}", cli.config);
        }
        None => {}
    }
}
