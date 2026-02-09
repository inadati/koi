# koi

Claude Code スキルパッケージマネージャー

gitライクなコマンドで、Claude Codeのスキルを取得・同期・公開できるCLIツールです。

## コンセプト

- 自分用のClaude Codeスキルを育てるツール
- Neovimプラグインを自分でカスタマイズするように、スキルも自分で成長させる
- clone/pull/pushでgitライクに管理

## インストール

### 前提条件

- [gh CLI](https://cli.github.com/)（GitHub CLI）
- [git](https://git-scm.com/)

```bash
gh auth login
```

### GitHub Releasesから

[Releases](https://github.com/inadati/koi/releases)からお使いの環境に合ったバイナリをダウンロードしてください。

```bash
tar xzf koi-v*.tar.gz
mv koi /usr/local/bin/
```

### ソースからビルド

```bash
cargo build --release
cp target/release/koi /usr/local/bin/
```

## セットアップ

```bash
# GitHub organizationをremoteとして登録
koi remote add <org-name>
```

## コマンド

| コマンド | 短縮 | 説明 |
|---------|------|------|
| `koi clone` | | スキルを取得（曖昧検索で選択） |
| `koi clone <name>` | | スキルを直接指定して取得 |
| `koi clone -r` | | `.koi.skills`から一括復元 |
| `koi clone -g` | | グローバルにクローン |
| `koi pull` | | 全スキルをリモートから同期 |
| `koi pull -g` | | グローバルスキルを同期 |
| `koi push` | | ローカル変更をリモートに反映 |
| `koi push -g` | | グローバルの変更を反映 |
| `koi new <name>` | | 新規スキルを作成 |
| `koi list` | `koi ls` | インストール済みスキル一覧 |
| `koi list -g` | `koi ls -g` | グローバルのスキル一覧 |
| `koi remove` | `koi rm` | スキルを削除（曖昧検索で選択） |
| `koi remove -g` | `koi rm -g` | グローバルのスキルを削除 |
| `koi remote add <org>` | | GitHub remoteを追加 |
| `koi remote remove` | `koi remote rm` | remoteを削除 |
| `koi remote list` | `koi remote ls` | remote一覧を表示 |
| `koi remote switch` | | remoteを切り替え |

## 使い方

```bash
# remoteを登録
koi remote add my-org

# スキルをクローン（曖昧検索UIで選択）
koi clone

# 全スキルを最新に同期
koi pull

# ローカルの変更をリモートに反映
koi push

# 新しいスキルを作成
koi new my-new-skill

# 別の環境で .koi.skills から一括復元
koi clone -r
```

## スキル管理ファイル

| ファイル | 用途 |
|---------|------|
| `.koi.skills` | プロジェクトローカルのスキル管理（リポジトリにコミット） |
| `~/.koi/global.skills` | グローバルスキルの管理 |
| `~/.koi/remotes.toml` | 登録済みGitHub remoteの管理 |

```toml
# .koi.skills
[skills]
moli = "my-org/moli"
expert-skill-make = "my-org/expert-skill-make"
```

## なぜ install/update ではなく clone/pull/push なのか

npm/cargoなどのパッケージマネージャーは、作者がpublishし利用者がinstallする**一方向**の関係です。koiでは同一人物がスキルの取得と公開を行う**双方向**の管理を行うため、gitのコマンド体系を採用しています。

## 技術スタック

- **Rust** + **clap**（CLI）
- **ratatui** + **crossterm** + **fuzzy-matcher**（対話的UI）
- **gh CLI** + **git**（GitHub API・リポジトリ操作）

## License

MIT
