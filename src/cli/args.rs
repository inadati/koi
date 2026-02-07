use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "koi", about = "Claude Code skill package manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// スキルをインストール
    #[command(alias = "i")]
    Install {
        /// スキル名（省略時は曖昧検索）
        name: Option<String>,
        /// グローバルにインストール
        #[arg(short, long)]
        global: bool,
        /// .koi.skillsから一括復元
        #[arg(short, long)]
        restore: bool,
    },
    /// スキルを更新（リモートからpull）
    #[command(alias = "u")]
    Update {
        /// グローバルのスキルを更新
        #[arg(short, long)]
        global: bool,
    },
    /// 新規スキルを作成
    New {
        /// スキル名
        name: String,
    },
    /// リモート操作
    #[command(alias = "r")]
    Remote {
        #[command(subcommand)]
        command: RemoteCommands,
    },
    /// インストール済みスキル一覧
    #[command(alias = "ls")]
    List {
        /// グローバルのスキル一覧
        #[arg(short, long)]
        global: bool,
    },
    /// スキルをアンインストール
    Uninstall {
        /// スキル名（省略時は曖昧検索）
        name: Option<String>,
        /// グローバルのスキルを削除
        #[arg(short, long)]
        global: bool,
    },
}

#[derive(Subcommand)]
pub enum RemoteCommands {
    /// ローカル変更をリモートに反映（push）
    #[command(alias = "u")]
    Update {
        /// グローバルのスキルをpush
        #[arg(short, long)]
        global: bool,
    },
    /// スキル検索対象のGitHub orgを設定
    SetOrg {
        /// org名
        org: String,
    },
}
