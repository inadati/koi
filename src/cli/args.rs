use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "koi", about = "Claude Code skill package manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// スキルを追加
    Add {
        /// スキル名（省略時は曖昧検索）
        name: Option<String>,
        /// グローバルに追加
        #[arg(short, long)]
        global: bool,
    },
    /// .koi.skillsから一括復元
    Restore {
        /// グローバルのスキルを復元
        #[arg(short, long)]
        global: bool,
    },
    /// リモートとローカルを同期
    Sync {
        /// グローバルのスキルを同期
        #[arg(short, long)]
        global: bool,
    },
    /// 新規スキルを作成
    New {
        /// スキル名
        name: String,
        /// リモートエイリアス名
        #[arg(short, long)]
        remote: Option<String>,
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
        /// エイリアス名
        alias: String,
        /// GitHub organization名
        org: String,
    },
    /// GitHub remoteを削除
    #[command(alias = "rm")]
    Remove {
        /// エイリアス名（省略時は曖昧検索）
        alias: Option<String>,
    },
    /// remote一覧を表示
    #[command(alias = "ls")]
    List,
    /// remoteのorg名を更新（GitHub org名が変わったときに使用）
    SetUrl {
        /// エイリアス名
        alias: String,
        /// 新しいGitHub organization名
        org: String,
    },
}
