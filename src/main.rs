// start auto exported by moli.
mod cli;
mod commands;
mod github;
mod git;
mod skill;
mod ui;
mod utils;
// end auto exported by moli.

use clap::Parser;

use cli::args::Cli;
use cli::commands::run;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
