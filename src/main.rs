mod list;
mod make;
mod select;
mod structs;

use crate::list::list;
use crate::make::make;
use crate::select::select;
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
        #[arg(short, long)]
        json: bool,
        #[arg(short, long)]
        value: bool,
    },
    Select {
        #[arg(short, long)]
        index: Option<usize>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Make => {
            make();
        }
        Commands::List { json, value } => {
            list(*json, *value);
        }
        Commands::Select { index } => {
            select(*index);
        }
    }
}
