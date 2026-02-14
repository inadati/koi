use clap::{Arg, Command, ValueEnum};
use clap_complete::{generate, Shell};

#[derive(Debug, Clone, ValueEnum)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl From<CompletionShell> for Shell {
    fn from(shell: CompletionShell) -> Self {
        match shell {
            CompletionShell::Bash => Shell::Bash,
            CompletionShell::Zsh => Shell::Zsh,
            CompletionShell::Fish => Shell::Fish,
            CompletionShell::PowerShell => Shell::PowerShell,
            CompletionShell::Elvish => Shell::Elvish,
        }
    }
}

pub fn run(shell: CompletionShell) {
    let mut cmd = build_cli();
    let bin_name = cmd.get_name().to_string();
    let shell_type: Shell = shell.into();
    generate(shell_type, &mut cmd, bin_name, &mut std::io::stdout());
}

// CLIの構造を再現
fn build_cli() -> Command {
    let version = env!("CARGO_PKG_VERSION");

    Command::new("koi")
        .about("Koi - スキルパッケージマネージャー")
        .version(version)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("clone")
                .about("スキルをクローン")
                .alias("c")
                .arg(Arg::new("name").help("スキル名"))
                .arg(
                    Arg::new("global")
                        .short('g')
                        .long("global")
                        .help("グローバルにインストール")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("restore")
                        .short('r')
                        .long("restore")
                        .help(".koi.skillsから一括復元")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("sync")
                .about("リモートとローカルを同期")
                .arg(
                    Arg::new("global")
                        .short('g')
                        .long("global")
                        .help("グローバルスキルを同期")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("new")
                .about("新規スキルを作成")
                .arg(Arg::new("name").help("スキル名").required(true)),
        )
        .subcommand(
            Command::new("remote")
                .about("GitHub remote管理")
                .subcommand_required(true)
                .subcommand(
                    Command::new("add")
                        .about("remoteを追加")
                        .arg(Arg::new("org").help("GitHub org名").required(true)),
                )
                .subcommand(
                    Command::new("remove")
                        .about("remoteを削除")
                        .alias("rm")
                        .arg(Arg::new("org").help("GitHub org名")),
                )
                .subcommand(
                    Command::new("list")
                        .about("remote一覧を表示")
                        .alias("ls"),
                )
                .subcommand(
                    Command::new("switch")
                        .about("remoteを切り替え")
                        .arg(Arg::new("org").help("GitHub org名")),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("インストール済みスキル一覧")
                .arg(
                    Arg::new("global")
                        .short('g')
                        .long("global")
                        .help("グローバルスキルを表示")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("スキルを削除")
                .alias("rm")
                .arg(Arg::new("name").help("スキル名"))
                .arg(
                    Arg::new("global")
                        .short('g')
                        .long("global")
                        .help("グローバルから削除")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("completion")
                .about("シェル補完スクリプトを生成")
                .long_about(
                    "シェル補完スクリプトを生成します。\n\n\
                    インストール例:\n  \
                    Bash:       koi completion bash > /usr/local/etc/bash_completion.d/koi\n  \
                    Zsh:        koi completion zsh > ~/.zsh/completion/_koi\n  \
                    Fish:       koi completion fish > ~/.config/fish/completions/koi.fish\n  \
                    PowerShell: koi completion powershell > koi.ps1",
                )
                .arg(
                    Arg::new("shell")
                        .help("シェルの種類")
                        .value_parser(clap::value_parser!(CompletionShell))
                        .required(true),
                ),
        )
}
