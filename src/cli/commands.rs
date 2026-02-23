use crate::cli::args::{Cli, Commands, RemoteCommands};
use crate::commands::{add, completion, list, new, remote, remove, restore, sync};
use crate::utils::error::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Add { name, global } => add::run(name, global),
        Commands::Restore { global } => restore::run(global),
        Commands::Sync { global } => sync::run(global),
        Commands::New {
            name,
            remote: remote_alias,
        } => new::run(&name, remote_alias),
        Commands::Remote { command } => match command {
            RemoteCommands::Add { alias, org } => remote::run_add(&alias, &org),
            RemoteCommands::Remove { alias } => remote::run_remove(alias),
            RemoteCommands::List => remote::run_list(),
            RemoteCommands::SetUrl { alias, org } => remote::run_set_url(&alias, &org),
        },
        Commands::List { global } => list::run(global),
        Commands::Remove { name, global } => remove::run(name, global),
        Commands::Completion { shell } => {
            completion::run(shell);
            Ok(())
        }
    }
}
