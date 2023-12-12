mod puzzle_inputs;

use crate::puzzle_inputs::get_puzzle_input;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Ensures that the input files for the current day are cached locally.
    Input {
        /// The day to cache the input for.
        day: u8,
    },
}

fn main() {
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Input { day }) => {
            let _ = get_puzzle_input(*day as i32, 1);
        }
        None => {}
    }
}
