use crate::cli::args::{Cli, Commands, RemoteCommands};
use crate::commands::{clone, list, new, pull, push, remote, remove};
use crate::utils::error::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Clone {
            name,
            global,
            restore,
        } => clone::run(name, global, restore),
        Commands::Pull { global } => pull::run(global),
        Commands::Push { global } => push::run(global),
        Commands::New { name } => new::run(&name),
        Commands::Remote { command } => match command {
            RemoteCommands::Add { org } => remote::run_add(&org),
            RemoteCommands::Remove { org } => remote::run_remove(org),
            RemoteCommands::List => remote::run_list(),
            RemoteCommands::Switch { org } => remote::run_switch(org),
        },
        Commands::List { global } => list::run(global),
        Commands::Remove { name, global } => remove::run(name, global),
    }
}
