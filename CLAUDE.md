# CLAUDE.md

このファイルは、Claude Codeがkoiリポジトリのコードを扱う際のガイダンスを提供します。

## プロジェクト概要

koiは、Claude Codeのスキルを管理するためのCLIパッケージマネージャーです。

### コンセプト

- 自分用のClaude Codeスキルを育てるツール
- Neovimプラグインを自分でカスタマイズするように、スキルも自分で成長させる
- clone（取得）、pull（同期）、push（反映）でgitライクに管理
- gitコマンドと統一感のある直感的な操作

### 技術スタック

- **言語**: Rust
- **CLI**: clap
- **対話的UI**: ratatui + crossterm + fuzzy-matcher
- **外部依存**: gh CLI（GitHub CLI）+ git
- **開発フレームワーク**: moli (v2.1.0)

## 開発方針

### moliフレームワークの使用

このプロジェクトは[moli](https://github.com/kyoheiu/moli)フレームワークを使用して開発します。

- `moli.yml`でプロジェクト構造を定義
- `moli up`でコード生成

### 技術方針: gh CLI + git

koiは**gh CLI**と**git**の両方に依存します。

- **gh CLI**: 認証、GitHub API操作（リポジトリ一覧・作成、org管理）
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

この特徴から、**gitとの統一感**を最優先したコマンド設計になっています：

```bash
koi clone     # スキルを取得（git clone）
koi pull      # 全スキルを同期（git stash + git pull）
koi push      # 全スキルを反映（git add + commit + push）
koi remote    # リモートリポジトリ管理（git remote相当）
```

### コマンド名の選定理由

- gitコマンドとの統一感を最優先
- スキル管理 = gitリポジトリ管理という明確な対応関係
- 学習コストの低減（gitユーザーにとって直感的）

### 設計原則

1. **gitとの統一感**: clone/pull/push/remoteのgitライクなコマンド体系
2. **シンプルさ**: 外部依存はgh CLI + gitのみ
3. **対話性**: ratauiによる曖昧検索UI
4. **プライベート前提**: gh authによる認証でプライベートリポジトリをサポート

### 実装の優先順位

1. **Phase 1**: 基本コマンド（clone、list、remove）+ gh CLI認証チェック + .koi.skills管理
2. **Phase 2**: 同期コマンド（pull、push）+ stash機能
3. **Phase 3**: remote管理の強化（複数remote対応）

## アーキテクチャ

### ディレクトリ構造

```
koi/
└── src/
    ├── main.rs           # CLIエントリーポイント
    ├── cli/
    │   ├── args.rs       # コマンドライン引数の定義
    │   └── commands.rs   # サブコマンドの列挙と実行
    ├── commands/
    │   ├── clone.rs      # スキルの取得 / --restoreで一括復元
    │   ├── pull.rs       # リモートからローカルへ同期
    │   ├── push.rs       # ローカル変更をリモートに反映
    │   ├── new.rs        # 新規スキル作成（リモートリポジトリ + SKILL.md + ローカルDL）
    │   ├── remote.rs     # GitHub remote管理（add/rm/list/switch）
    │   ├── list.rs       # インストール済みスキルの一覧
    │   └── remove.rs     # スキルの削除
    ├── github/
    │   ├── api.rs        # gh apiコマンドのラッパー
    │   ├── repo.rs       # リポジトリ一覧取得・作成
    │   └── auth.rs       # gh CLI認証チェック
    ├── git/
    │   ├── command.rs    # git -C によるコマンド実行ラッパー
    │   ├── clone.rs      # git clone
    │   └── sync.rs       # git stash / pull / push --no-verify
    ├── skill/
    │   ├── metadata.rs   # スキルのメタデータ（SKILL.md解析）
    │   ├── path.rs       # スキルのパス解決
    │   ├── validator.rs  # スキルの妥当性検証
    │   └── lockfile.rs   # .koi.skills / ~/.koi/global.skills の読み書き
    ├── ui/
    │   ├── fuzzy.rs      # ratatui + fuzzy-matcherによる曖昧検索UI
    │   ├── prompt.rs     # 確認プロンプト
    │   └── progress.rs   # 進行状況表示
    └── utils/
        ├── fs.rs         # ファイルシステム操作
        ├── error.rs      # エラー型の定義
        └── config.rs     # 設定ファイルの読み書き
```

### レイヤー構成

1. **cli層**: clapによるCLIインターフェース定義
2. **commands層**: 各サブコマンドの実装
3. **github層**: gh CLI（GitHub API操作）
4. **git層**: gitコマンドのラッパー（clone、pull、push、stash）
5. **skill層**: スキル管理ロジック
6. **ui層**: ratauiによる対話的UI
7. **utils層**: 共通ユーティリティ

## 設定ファイル

### ~/.koi/config.toml

```toml
[paths]
# デフォルトのインストールパス
local = ".claude/skills"
global = "~/.claude/skills"
```

**認証**: gh CLI（`gh auth login`）に一任するため、設定ファイルに認証情報は持たない。

## コマンド詳細

### koi clone

スキルをリモートリポジトリから取得します。

```bash
koi clone              # 曖昧検索で選択してクローン（.koi.skillsに追記）
koi clone <skill-name> # 直接指定
koi clone -g           # グローバル（~/.claude/skills）にクローン
koi clone --restore    # .koi.skillsから一括復元（koi clone -r）
```

**動作（通常）**:
1. `gh api` でリモートリポジトリのスキル一覧を取得
2. ratauiの曖昧検索UIで選択
3. `git clone` で `.claude/skills/` にクローン
4. `.koi.skills` にスキル情報を追記

**動作（--restore）**:
1. `.koi.skills` を読み込み
2. 記載された全スキルを `git clone` で `.claude/skills/` にクローン

**認証**: gh CLI（`gh auth login`）に一任。プライベートリポジトリ対応済み。

### koi pull

リモートからローカルへスキルを同期します。

```bash
koi pull       # プロジェクトローカルの全スキルを同期
koi pull -g    # グローバルの全スキルを同期
```

**動作**:
1. 全インストール済みスキルに対して（`git -C` でディレクトリ指定）:
   a. ローカルに未pushの変更がある場合 → `git stash` で退避
   b. `git pull` でリモートから取得
2. stashした場合はその旨を通知

**リモートが常に正。コンフリクトは発生しない。**

**stash時の出力**:
```
Warning: スキル "moli" にローカル変更がありました → git stashで退避しました
```

### koi push

ローカルの変更をリモートに反映します。

```bash
koi push       # プロジェクトローカルの変更をリモートに反映
koi push -g    # グローバルの変更をリモートに反映
```

**動作**:
1. 全インストール済みスキルに対して（`git -C` でディレクトリ指定）:
   a. `git add .` + `git commit --no-verify -m "update skill"`
   b. `git push --no-verify`
   c. pushが失敗した場合（リモートが先行）→ エラーを出し `koi pull` を先に実行するよう案内

**push失敗時の出力**:
```
Error: スキル "moli" のリモートが別の環境で更新されています
  先に koi pull を実行してください
```

### koi new

リモートに新規スキルリポジトリを作成し、ローカルにダウンロードします。

```bash
koi new <skill-name>    # 新規スキル作成
```

**動作**:
1. `gh api` で設定済みorgに新規プライベートリポジトリを作成
2. `gh api` でSKILL.mdテンプレートをリポジトリに配置
3. `git clone` でローカル（`.claude/skills/<skill-name>/`）にクローン
4. `.koi.skills` に追記

### koi remote

GitHub remote（organization）を管理します。

```bash
koi remote add <org>         # GitHub remoteを追加
koi remote remove            # 曖昧検索で選択して削除（koi remote rm）
koi remote remove <org>      # 直接指定して削除
koi remote list              # remote一覧を表示（koi remote ls）
koi remote switch            # 曖昧検索で選択して切り替え
koi remote switch <org>      # 直接指定して切り替え
```

**動作**:
- `~/.koi/remotes.toml` でremote（org）情報を管理
- 複数remoteの登録・切り替えが可能
- gitの`git remote`コマンドと同様の操作感
- `remove`と`switch`はratauiの曖昧検索UIで選択可能（`koi clone`と同様）

### koi list

インストール済みのスキル一覧を表示します。

```bash
koi list       # ローカルのスキル一覧
koi list -g    # グローバルのスキル一覧
```

**動作**:
1. `.claude/skills/` または `~/.claude/skills/` のディレクトリを走査
2. スキル名、バージョン、説明などを表示

### koi remove (`koi rm`)

スキルを削除します。

```bash
koi remove              # 曖昧検索で選択して削除
koi remove <skill-name> # 直接指定（koi rm <skill-name>）
koi remove -g           # グローバルのスキルを削除
```

**動作**:
1. インストール済みスキルをリスト化
2. ratauiの曖昧検索UIで選択
3. 確認プロンプトを表示
4. ディレクトリを削除
5. `.koi.skills` から該当スキルを削除

### コマンド一覧表

| コマンド | 短縮 | 説明 | 対話的UI |
|---------|------|------|----------|
| `koi clone` | | スキルを取得 | 曖昧検索で選択 |
| `koi clone --restore` | `koi clone -r` | .koi.skillsから一括復元 | - |
| `koi clone -g` | | グローバルにクローン | 曖昧検索で選択 |
| `koi pull` | | リモートからローカルへ同期 | - |
| `koi pull -g` | | グローバルスキルを同期 | - |
| `koi push` | | ローカル変更をリモートに反映 | - |
| `koi push -g` | | グローバルの変更をリモートに反映 | - |
| `koi new <skill-name>` | | 新規スキル作成（リモートリポジトリ + SKILL.md + ローカルDL） | - |
| `koi list` | | インストール済みスキル一覧 | - |
| `koi list -g` | | グローバルのスキル一覧 | - |
| `koi remove` | `koi rm` | スキルを削除 | 曖昧検索で選択 |
| `koi remove -g` | `koi rm -g` | グローバルのスキルを削除 | 曖昧検索で選択 |
| `koi remote add <org>` | | GitHub remoteを追加 | - |
| `koi remote remove` | `koi remote rm` | GitHub remoteを削除 | 曖昧検索で選択 |
| `koi remote list` | `koi remote ls` | remote一覧を表示 | - |
| `koi remote switch` | | remoteを切り替え | 曖昧検索で選択 |
| `koi completion <shell>` | | シェル補完スクリプトを生成（bash/zsh/fish/powershell/elvish） | - |

### スキル管理ファイル

| ファイル | 用途 |
|---------|------|
| `.koi.skills` | プロジェクトローカルのスキル管理。リポジトリにコミットする。`koi clone -r` で一括復元可能。 |
| `~/.koi/global.skills` | グローバルスキルのリモートマッピング。pull/push用。 |
| `~/.koi/remotes.toml` | 登録済みGitHub remoteの管理。アクティブなremoteも記録。 |

```toml
# .koi.skills / ~/.koi/global.skills（同フォーマット）
[skills]
moli = "itton-claude-skills/moli"
expert-skill-make = "itton-claude-skills/expert-skill-make"
```

```toml
# ~/.koi/remotes.toml
active = "itton-claude-skills"

[remotes]
itton-claude-skills = { description = "個人スキルリポジトリ" }
work-org = { description = "業務用スキルリポジトリ" }
```

### シェル補完のインストール

`koi completion`コマンドでシェル補完スクリプトを生成できます。

```bash
# Bash
koi completion bash > /usr/local/etc/bash_completion.d/koi

# Zsh
koi completion zsh > ~/.zsh/completion/_koi
# または
koi completion zsh > /usr/local/share/zsh/site-functions/_koi

# Fish
koi completion fish > ~/.config/fish/completions/koi.fish

# PowerShell
koi completion powershell > koi.ps1
```

インストール後、シェルを再起動するか、補完ファイルをリロードしてください。

### なぜ search, init がないのか

- **search**: `koi clone`が曖昧検索機能を持つため不要
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
- gh CLI: 認証、GitHub API操作（リポジトリ一覧・作成、org管理）
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
- ui層: fuzzy.rs（ratatui + fuzzy-matcher曖昧検索）, prompt.rs（y/n確認）, progress.rs（進行状況表示）
- cli層: args.rs（clap定義）, commands.rs（ディスパッチ）
- commands層: clone.rs（曖昧検索+--restore）, list.rs, remove.rs, remote.rs（add/rm/list/switch）
- main.rs

**スタブ（Phase 2で実装予定）**:
- commands/pull.rs - `koi pull`（git stash + git pull）
- commands/push.rs - `koi push`（git add + commit + push）
- commands/new.rs - `koi new`（リモートリポジトリ作成 + clone）

**Phase 3（実装済み）**:
- ui層をratauiベースの曖昧検索UIに実装済み（fuzzy-matcherによるスコアリング）

**技術的決定事項**:
- clone URLはHTTPS（`https://github.com/{org}/{repo}.git`）を使用
- `gh auth setup-git` でHTTPS pushの認証を確保
- gitの操作はすべて `git -C <dir>` で対象ディレクトリを指定

### 2026-02-09: コマンド体系の大幅変更

- **gitライクなコマンド体系への変更**
- **コマンド変更**:
  - `koi install` → `koi clone`（スキルの取得）
  - `koi uninstall` → `koi remove` (短縮: `rm`)（スキルの削除）
  - `koi update` → `koi pull`（リモートからローカルへ同期）
  - `koi org update` → `koi push`（ローカル変更をリモートに反映）
  - `koi org` → `koi remote`（remote管理）
- **remote管理の強化**:
  - 複数remote対応（add/remove/list/switch）
  - `koi remote rm` → `koi remote remove` (短縮: `rm`)
  - `koi remote remove`と`koi remote switch`で曖昧検索UI対応
- MOLI.mdの内容をCLAUDE.mdに統合（情報の一元化）
- 技術スタックの明確化: dioxus → ratatui + crossterm + fuzzy-matcher
- 設定ファイル名の変更: `~/.koi/orgs.toml` → `~/.koi/remotes.toml`

### 2026-02-10: シェル補完機能の追加 (v0.1.1)

- **シェル補完機能の実装**
- `clap_complete`クレートを使用
- `koi completion <shell>`コマンドを追加
  - 対応シェル: bash, zsh, fish, powershell, elvish
- moliの実装を参考に、CLIの全コマンド構造を再現
- CLAUDE.mdにインストール方法を追記
- バージョンを0.1.0から0.1.1に更新

## 参考資料

- [cargo](https://github.com/rust-lang/cargo)
- [mise](https://github.com/jdx/mise)
- [npm](https://github.com/npm/cli)
- [helm](https://github.com/helm/helm)
- [homebrew](https://github.com/Homebrew/brew)
