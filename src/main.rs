use std::{any, env};

use clap::{Parser, Subcommand};

mod app;
mod components;
mod messages;

use crate::app::app;

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

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    match args.len() {
        1 => {
            app()?;

            Ok(())
        }

        _ => {
            // let cli = Cli::parse();

            // match cli.commands {
            //     Cmds::Test => Ok::<(), anyhow::Error>(()),
            // };

            Ok(())
        }
    }
}
