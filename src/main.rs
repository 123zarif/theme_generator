mod list;
mod make;

use crate::list::list;
use crate::make::make;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Make,
    List {
        #[arg(short, long, global = true)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Make => {
            make();
        }
        Commands::List { json } => {
            list(*json);
        }
    }
}
