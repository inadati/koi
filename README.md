# koi

Claude Code ユニークスキル パッケージマネージャー

自分専用のClaude Codeスキル（ユニークスキル）を育てるための環境を提供し、gitライクなコマンドで管理するCLIツールです。  
GitHub organizationにスキル用のリポジトリを作成し、そこを自分専用のスキル置き場とすることができます。

## コンセプト

- ゲームのRPGでキャラ育成を楽しむように
- 自分のClaude Codeを継続的にかつ個性的に育成・強化していくことを可能にし、
- claude codeユーザーにRPGをプレイしているかのような楽しいスキル開発体験を提供します。

## インストール

### 前提条件

- [gh CLI](https://cli.github.com/)（GitHub CLI）
- [git](https://git-scm.com/)

```bash
gh auth login
```

### インストールスクリプト

```bash
curl -fsSL https://raw.githubusercontent.com/inadati/koi/main/install.sh | sh
```

`~/.local/bin/koi` にインストールされます。PATHの設定が必要な場合はスクリプト実行後に案内が表示されます。

### ソースからビルド

```bash
cargo build --release
cp target/release/koi ~/.local/bin/
```

## セットアップ

> **注意**: koiの`sync`コマンドはmainブランチに強制的にpushします。**必ずスキル管理専用のGitHub organizationを作成してください。** 既存のorganizationやリポジトリを登録すると、意図しない変更がpushされる恐れがあります。

GitHubにスキル専用のorganizationを作成し、remoteとして登録します。

```
https://github.com/<org>/<repo>
                    ^^^
                    ここがorg名
```

```bash
koi remote add <alias> <org>
```

```bash
# 例: 基本スキル用のremoteを登録
koi remote add basic my-basic-skills
#               ^^^^^ ^^^^^^^^^^^^^^^^
#               エイリアス名  GitHub org名

# 複数のremoteを登録する場合
koi remote add engineer my-engineer-skills
```

## コマンド

| コマンド | 短縮 | 説明 |
|---------|------|------|
| `koi add` | | 全リモートを横断してスキルを追加（曖昧検索） |
| `koi add <name>` | | スキルを直接指定して追加 |
| `koi add -g` | | グローバルに追加 |
| `koi restore` | | `.koi.skills`から一括復元 |
| `koi restore -g` | | グローバルのスキルを復元 |
| `koi sync` | | 全スキルをリモートと同期（pull + push） |
| `koi sync -g` | | グローバルスキルを同期 |
| `koi new <name>` | | 新規スキルを作成（リモートリポジトリ + ローカルDL） |
| `koi new <name> -r <alias>` | | リモートを指定して新規作成 |
| `koi list` | `koi ls` | インストール済みスキル一覧 |
| `koi list -g` | `koi ls -g` | グローバルのスキル一覧 |
| `koi remove` | `koi rm` | スキルを削除（曖昧検索で選択） |
| `koi remove -g` | `koi rm -g` | グローバルのスキルを削除 |
| `koi remote add <alias> <org>` | | GitHub remoteを追加 |
| `koi remote remove` | `koi remote rm` | remoteを削除 |
| `koi remote list` | `koi remote ls` | remote一覧を表示 |
| `koi remote set-url <alias> <org>` | | remoteのorg名を更新 |
| `koi completion <shell>` | | シェル補完スクリプトを生成 |

## 使い方

```bash
# スキルを追加（全リモートから曖昧検索UIで選択）
koi add

# 全スキルをリモートと同期（プロジェクトルートで実行）
koi sync

# 新しいスキルを作成
koi new my-new-skill

# 別の環境で .koi.skills から一括復元
koi restore
```

## スキル管理ファイル

| ファイル | 用途 |
|---------|------|
| `.koi.skills` | プロジェクトローカルのスキル管理（リポジトリにコミット） |
| `~/.koi/global.skills` | グローバルスキルの管理 |
| `~/.koi/remotes.toml` | 登録済みGitHub remoteの管理（エイリアス名 → org名） |

```toml
# .koi.skills（値はエイリアス名）
[skills]
git-commit = "basic"
moli = "engineer"
```

```toml
# ~/.koi/remotes.toml
[remotes.basic]
org = "my-basic-skills"

[remotes.engineer]
org = "my-engineer-skills"
```

## なぜ clone/pull/push ではなく add/restore/sync なのか

npm/cargoなどのパッケージマネージャーは、作者がpublishし利用者がinstallする**一方向**の関係です。koiでは同一人物がスキルの取得と公開を行う**双方向**の管理を行います。`sync`がpull + pushを統合し、`add`で全リモートを横断検索、`restore`で環境の一括復元を行います。

## シェル補完

```bash
# Zsh
koi completion zsh > ~/.zsh/completion/_koi

# Bash
koi completion bash > /usr/local/etc/bash_completion.d/koi

# Fish
koi completion fish > ~/.config/fish/completions/koi.fish
```

## 技術スタック

- **Rust** + **clap**（CLI）
- **ratatui** + **crossterm** + **fuzzy-matcher**（対話的UI）
- **gh CLI** + **git**（GitHub API・リポジトリ操作）

## License

本ソフトウェア（koi CLI）は [MIT](LICENSE) ライセンスで提供されています。

## 関連プロジェクト

### koiちゃんはAIを語りたい

koi CLIの擬人化キャラクター「koiちゃん」がAIについて語るコンテンツプロジェクトです。note.comで記事を配信しています。

> **著作権について**: koiちゃんのキャラクター（名称・デザイン・設定・イラスト等）に関する一切の権利はAsweedに帰属します。無断での使用・複製・改変・再配布を禁じます。本リポジトリのMITライセンスはkoi CLIのソースコードにのみ適用され、キャラクターには適用されません。
