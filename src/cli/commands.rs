use crate::cli::args::{Cli, Commands, RemoteCommands};
use crate::commands::{install, list, new, remote, uninstall, update};
use crate::utils::error::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Install {
            name,
            global,
            restore,
        } => install::run(name, global, restore),
        Commands::Update { global } => update::run(global),
        Commands::New { name } => new::run(&name),
        Commands::Remote { command } => match command {
            RemoteCommands::Update { global } => remote::run_update(global),
            RemoteCommands::SetOrg { org } => remote::run_set_org(&org),
        },
        Commands::List { global } => list::run(global),
        Commands::Uninstall { name, global } => uninstall::run(name, global),
    }
}
