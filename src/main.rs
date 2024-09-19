use std::env;

use clap::{Parser, Subcommand};

mod app;
mod components;

use crate::app::dashboard;

#[derive(Debug, Subcommand)]
enum Cmds {
    /// Does stuff
    Test,
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Cmds,
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    match args.len() {
        1 => {
            dashboard();
        }

        _ => {
            let cli = Cli::parse();

            match cli.commands {
                Cmds::Test => todo!(),
            }
        }
    }
}
