use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "koi", about = "Claude Code skill package manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// スキルをクローン
    Clone {
        /// スキル名（省略時は曖昧検索）
        name: Option<String>,
        /// グローバルにクローン
        #[arg(short, long)]
        global: bool,
        /// .koi.skillsから一括復元
        #[arg(short, long)]
        restore: bool,
    },
    /// スキルを同期（リモートからpull）
    Pull {
        /// グローバルのスキルを同期
        #[arg(short, long)]
        global: bool,
    },
    /// ローカル変更をリモートに反映（push）
    Push {
        /// グローバルのスキルをpush
        #[arg(short, long)]
        global: bool,
    },
    /// 新規スキルを作成
    New {
        /// スキル名
        name: String,
    },
    /// リモート操作
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
    /// スキルを削除
    #[command(alias = "rm")]
    Remove {
        /// スキル名（省略時は曖昧検索）
        name: Option<String>,
        /// グローバルのスキルを削除
        #[arg(short, long)]
        global: bool,
    },
    /// シェル補完スクリプトを生成
    Completion {
        /// シェルの種類
        shell: crate::commands::completion::CompletionShell,
    },
}

#[derive(Subcommand)]
pub enum RemoteCommands {
    /// GitHub remoteを追加
    Add {
        /// org名
        org: String,
    },
    /// GitHub remoteを削除
    #[command(alias = "rm")]
    Remove {
        /// org名（省略時は曖昧検索）
        org: Option<String>,
    },
    /// remote一覧を表示
    #[command(alias = "ls")]
    List,
    /// remoteを切り替え
    Switch {
        /// org名（省略時は曖昧検索）
        org: Option<String>,
    },
}
