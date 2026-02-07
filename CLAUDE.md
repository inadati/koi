# CLAUDE.md

このファイルは、Claude Codeがkoiリポジトリのコードを扱う際のガイダンスを提供します。

## プロジェクト概要

koiは、Claude Codeのスキルを管理するためのCLIパッケージマネージャーです。

- **言語**: Rust
- **CLI**: clap
- **対話的UI**: dioxus
- **外部依存**: gh CLI（GitHub CLI）+ git
- **開発フレームワーク**: moli (v2.1.0)

## 開発方針

### moliフレームワークの使用

このプロジェクトは[moli](https://github.com/kyoheiu/moli)フレームワークを使用して開発します。

- `moli.yml`でプロジェクト構造を定義
- `moli up`でコード生成
- 詳細な仕様は`MOLI.md`を参照

### 技術方針: gh CLI + git

koiは**gh CLI**と**git**の両方に依存します。

- **gh CLI**: 認証、GitHub API操作（リポジトリ一覧・作成、org設定）
- **git**: ローカルリポジトリ操作（clone、pull、push、stash）
- 認証は `gh auth login` で完結（gitの認証もghが管理）
- githooksの制約は `--no-verify` で回避

```bash
# gh: GitHub API操作
gh api /orgs/{org}/repos                        # リポジトリ一覧
gh api /orgs/{org}/repos --method POST          # リポジトリ作成

# git: ローカルリポジトリ操作（-Cでディレクトリ指定）
git -C .claude/skills/moli stash
git -C .claude/skills/moli pull
git -C .claude/skills/moli push --no-verify
```

### コマンド設計の原則

koiのコマンド設計は、以下の特徴に基づいています：

**他のパッケージマネージャーとの違い**:
- npm/cargo: 作者（publish） ← 利用者（install/update）の**一方向**
- koi: 同一人物がpull/pushを行う**双方向**の管理

この特徴から、**gitとの統一感**を重視したコマンド設計になっています：

```bash
koi install   # スキルを取得（gh repo clone）
koi update    # 全スキルを同期（git stash + git pull）
```

### コマンド名の選定理由

- パッケージマネージャーとしての使用感を重視（install/update）
- 内部的にはローカル⇔リモートの双方向同期を行うが、ユーザーには馴染みのあるコマンド名を提供
- `update`は全スキルを一斉にpull（未pushの変更はgit stashで退避）

## コマンド一覧

| コマンド | 短縮 | 説明 | 対話的UI |
|---------|------|------|----------|
| `koi install` | `koi i` | スキルを取得 | 曖昧検索で選択 |
| `koi install --restore` | `koi i -r` | .koi.skillsから一括復元 | - |
| `koi install -g` | `koi i -g` | グローバルにインストール | 曖昧検索で選択 |
| `koi update` | `koi u` | リモートからローカルへ同期（pull） | - |
| `koi update -g` | `koi u -g` | グローバルスキルを同期 | - |
| `koi new <skill-name>` | | 新規スキル作成（リモートリポジトリ + SKILL.md + ローカルDL） | - |
| `koi remote update` | `koi r u` | ローカル変更をリモートに反映（push） | - |
| `koi remote update -g` | `koi r u -g` | グローバルの変更をリモートに反映 | - |
| `koi remote set-org <org>` | | スキル検索対象のGitHub orgを設定 | - |
| `koi list` | | インストール済みスキル一覧 | - |
| `koi uninstall` | | スキルを削除 | 曖昧検索で選択 |
| `koi uninstall -g` | | グローバルのスキルを削除 | 曖昧検索で選択 |

### updateの動作

1. ローカルに未pushの変更がある場合 → stashに退避
2. リモートの内容でローカルを上書き（リモートが常に正）
3. stashした旨を通知

※ コンフリクトは発生しない。リモートが常に正。

### スキル管理ファイル

| ファイル | 用途 |
|---------|------|
| `.koi.skills` | プロジェクトローカルのスキル管理。リポジトリにコミットする。`koi i -r` で一括復元可能。 |
| `~/.koi/global.skills` | グローバルスキルのリモートマッピング。update/remote update用。 |

```toml
# .koi.skills / ~/.koi/global.skills（同フォーマット）
[skills]
moli = "itton-claude-skills/moli"
expert-skill-make = "itton-claude-skills/expert-skill-make"
```

### なぜ search, init がないのか

- **search**: `koi install`が曖昧検索機能を持つため不要
- **init**: `expert-skill-make`など既存のClaude Codeスキルがあるため不要

## 認証

- **gh CLI（`gh auth login`）に一任**
- プライベートリポジトリ対応済み（gh authが認証を管理）
- gh/gitが未インストールの場合はインストールを案内する

## 開発履歴

### 2026-02-06: プロジェクト初期設計

- moli.ymlの設計と作成
- コマンド構成の検討（clone/pull/push/list/remove）
- MOLI.mdに詳細仕様を記録
- アーキテクチャの設計（6層構成）

### 2026-02-06: 技術方針の決定

- **gh CLI + git** に依存する方針に決定
- gh CLI: 認証、GitHub API操作（リポジトリ一覧・作成）
- git: ローカルリポジトリ操作（clone、pull、push、stash）
- githooksは `--no-verify` で回避
- 認証はgh auth loginに一任
- R2案を検討したが、GitHub APIの方がメタデータ管理が不要で適切と判断

**レイヤー構成**:
1. cli層: clapによるCLI定義
2. commands層: サブコマンドの実装
3. github層: gh CLI（GitHub API操作）
4. git層: gitコマンドのラッパー（clone、pull、push、stash）
5. skill層: スキル管理ロジック
6. ui層: dioxusによる対話的UI
7. utils層: 共通ユーティリティ

### 2026-02-06: Phase 1 実装完了

**実装済み（Phase 1）**:
- utils層: error.rs, config.rs, fs.rs
- skill層: path.rs, lockfile.rs, metadata.rs, validator.rs
- github層: auth.rs（gh auth setup-git含む）, api.rs, repo.rs
- git層: command.rs, clone.rs（HTTPS URL）, sync.rs
- ui層: fuzzy.rs（番号選択）, prompt.rs（y/n確認）, progress.rs（println!ベース）
- cli層: args.rs（clap定義）, commands.rs（ディスパッチ）
- commands層: install.rs（曖昧検索+--restore）, list.rs, uninstall.rs, remote.rs（set-org）
- main.rs

**スタブ（Phase 2で実装予定）**:
- commands/update.rs - `koi update`（git stash + git pull）
- commands/new.rs - `koi new`（リモートリポジトリ作成 + clone）
- commands/remote.rs の `run_update` - `koi remote update`（git add + commit + push）

**Phase 3で実装予定**:
- ui層をdioxusベースの曖昧検索UIに置き換え（現在は番号選択の簡易UI）

**技術的決定事項**:
- clone URLはHTTPS（`https://github.com/{org}/{repo}.git`）を使用
- `gh auth setup-git` でHTTPS pushの認証を確保
- gitの操作はすべて `git -C <dir>` で対象ディレクトリを指定
