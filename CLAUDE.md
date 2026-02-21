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
- koi: 同一人物がsyncを行う**双方向**の管理

この特徴から、**gitとの統一感**を最優先したコマンド設計になっています：

```bash
koi add       # スキルを取得（全リモートを横断検索）
koi restore   # .koi.skillsから一括復元
koi sync      # リモートとローカルを同期（pull + push統合）
koi remote    # リモートリポジトリ管理（git remote相当）
```

### コマンド名の選定理由

- gitコマンドとの統一感を最優先
- スキル管理 = gitリポジトリ管理という明確な対応関係
- 学習コストの低減（gitユーザーにとって直感的）

### 設計原則

1. **gitとの統一感**: add/restore/sync/remoteのgitライクなコマンド体系
2. **シンプルさ**: 外部依存はgh CLI + gitのみ
3. **対話性**: ratauiによる曖昧検索UI
4. **プライベート前提**: gh authによる認証でプライベートリポジトリをサポート

### 実装の優先順位

1. **Phase 1**: 基本コマンド（clone、list、remove）+ gh CLI認証チェック + .koi.skills管理
2. **Phase 2**: 同期コマンド（sync）+ stash機能
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
    │   ├── add.rs        # スキルの追加（全リモート横断検索）
    │   ├── restore.rs    # .koi.skillsから一括復元
    │   ├── sync.rs       # リモートとローカルを同期（pull + push統合）
    │   ├── new.rs        # 新規スキル作成（リモートリポジトリ + SKILL.md + ローカルDL）
    │   ├── remote.rs     # GitHub remote管理（add/rm/list/set-url）
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

### koi add

登録済みの全リモートを横断してスキルを追加します。

```bash
koi add              # 全リモートから曖昧検索で選択して追加（複数選択可）
koi add <skill-name> # 直接指定（全リモートを横断して検索、AI用）
koi add -g           # グローバル（~/.claude/skills）に追加
```

**動作**:
1. 登録済み全リモートから `gh api` でリポジトリ一覧を収集
2. インストール済み（ローカル・グローバル両方）を除外
3. ratauiの曖昧検索UIで選択（表示形式: `skill-name @alias`）
4. `git clone` で `.claude/skills/` にクローン
5. `.koi.skills` にエイリアス名を記録

**認証**: gh CLI（`gh auth login`）に一任。プライベートリポジトリ対応済み。

### koi restore

`.koi.skills` からスキルを一括復元します。

```bash
koi restore      # ローカルの.koi.skillsから一括復元
koi restore -g   # グローバルから一括復元
```

**動作**:
1. `.koi.skills` を読み込み（スキル名 → エイリアス名のマッピング）
2. エイリアス名から org 名を解決（`remotes.toml` を参照）
3. 記載された全スキルを `git clone` で `.claude/skills/` にクローン

**認証**: gh CLI（`gh auth login`）に一任。プライベートリポジトリ対応済み。

### koi sync

リモートとローカルを同期します（pull + push統合）。

```bash
koi sync       # プロジェクトローカルの全スキルを同期
koi sync -g    # グローバルの全スキルを同期
```

**動作**:
1. 全インストール済みスキルに対して（`git -C` でディレクトリ指定）:
   a. **ローカル変更がない場合**: `git pull` のみ実行
   b. **ローカル変更がある場合**:
      1. `git stash` でローカル変更を退避
      2. `git pull` でリモートの最新を取得
      3. `git stash pop` でローカル変更を再適用
      4. コンフリクトが発生した場合 → 警告を表示して次のスキルへ
      5. `git add -A` + `git commit --no-verify` + `git push --no-verify`

**コンフリクト時の出力**:
```
Warning: スキル "moli" でコンフリクトが発生しました
  手動で解決してください: .claude/skills/moli/
```

### koi new

リモートに新規スキルリポジトリを作成し、ローカルにダウンロードします。

```bash
koi new <skill-name>             # 新規スキル作成（リモートをfuzzy UIで選択）
koi new <skill-name> -r <alias>  # リモートを直接指定して作成
```

**動作**:
1. リモートエイリアスを決定（`--remote` 未指定時: 1つなら自動選択、複数ならfuzzy UI）
2. `gh api` でorgに新規プライベートリポジトリを作成
3. `gh api` でSKILL.mdテンプレートをリポジトリに配置
4. `git clone` でローカル（`.claude/skills/<skill-name>/`）にクローン
5. `.koi.skills` にエイリアス名を記録

### koi remote

GitHub remote（organization）を管理します。

```bash
koi remote add <org>                # GitHub remoteを追加（エイリアス名をプロンプトで入力）
koi remote add <org> --name <alias> # エイリアス名を直接指定して追加
koi remote remove                   # 曖昧検索で選択して削除（koi remote rm）
koi remote remove <alias>           # 直接指定して削除
koi remote list                     # remote一覧を表示（koi remote ls）
koi remote set-url <alias> <org>    # エイリアスのorg名を更新（GitHub org改名時に使用）
```

**動作**:
- `~/.koi/remotes.toml` でremote（alias → org）情報を管理
- エイリアス名: 英数字・ハイフン・アンダースコアのみ（デフォルトはorg名）
- `koi remote remove` で参照中エイリアスがある場合は警告を表示（削除は続行）
- `koi remote set-url` は `remotes.toml` のorg値のみ更新（`.koi.skills` の変更は不要）

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
| `koi add` | | 全リモートを横断してスキルを追加（複数選択可） | 曖昧検索で選択 |
| `koi add -g` | | グローバルに追加 | 曖昧検索で選択 |
| `koi restore` | | .koi.skillsから一括復元 | - |
| `koi restore -g` | | グローバルのスキルを復元 | - |
| `koi sync` | | リモートとローカルを同期 | - |
| `koi sync -g` | | グローバルスキルを同期 | - |
| `koi new <skill-name>` | | 新規スキル作成（リモートリポジトリ + SKILL.md + ローカルDL） | リモート選択 |
| `koi new <skill-name> -r <alias>` | | リモートを直接指定して新規スキル作成 | - |
| `koi list` | | インストール済みスキル一覧 | - |
| `koi list -g` | `koi ls -g` | グローバルのスキル一覧 | - |
| `koi remove` | `koi rm` | スキルを削除 | 曖昧検索で選択 |
| `koi remove -g` | `koi rm -g` | グローバルのスキルを削除 | 曖昧検索で選択 |
| `koi remote add <org>` | | GitHub remoteを追加（エイリアス名をプロンプトで入力） | - |
| `koi remote remove` | `koi remote rm` | GitHub remoteを削除 | 曖昧検索で選択 |
| `koi remote list` | `koi remote ls` | remote一覧を表示 | - |
| `koi remote set-url <alias> <org>` | | エイリアスのorg名を更新 | - |
| `koi completion <shell>` | | シェル補完スクリプトを生成（bash/zsh/fish/powershell/elvish） | - |

### スキル管理ファイル

| ファイル | 用途 |
|---------|------|
| `.koi.skills` | プロジェクトローカルのスキル管理。リポジトリにコミットする。`koi restore` で一括復元可能。 |
| `~/.koi/global.skills` | グローバルスキルのリモートマッピング。sync用。 |
| `~/.koi/remotes.toml` | 登録済みGitHub remoteの管理（エイリアス名 → org名）。 |

```toml
# .koi.skills / ~/.koi/global.skills（同フォーマット）
# 値はエイリアス名（remotes.tomlのキー）
[skills]
moli = "personal"
expert-skill-make = "personal"
```

```toml
# ~/.koi/remotes.toml
# active フィールドなし（koi remote switch 廃止済み）
[remotes.personal]
org = "itton-claude-skills"

[remotes.work]
org = "work-org"
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

- **search**: `koi add`が全リモート横断の曖昧検索機能を持つため不要
- **init**: `expert-skill-make`など既存のClaude Codeスキルがあるため不要

## 認証

- **gh CLI（`gh auth login`）に一任**
- プライベートリポジトリ対応済み（gh authが認証を管理）
- gh/gitが未インストールの場合はインストールを案内する

## 開発履歴

### 2026-02-06: プロジェクト初期設計

- moli.ymlの設計と作成
- コマンド構成の検討（clone/sync/list/remove）
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
- commands/sync.rs - `koi sync`（pull + push統合）
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
  - `koi update` / `koi org update` → `koi sync`（リモートとローカルを同期）
  - `koi org` → `koi remote`（remote管理）
- **remote管理の強化**:
  - 複数remote対応（add/remove/list/switch）
  - `koi remote rm` → `koi remote remove` (短縮: `rm`)
  - `koi remote remove`と`koi remote switch`で曖昧検索UI対応
- MOLI.mdの内容をCLAUDE.mdに統合（情報の一元化）
- 技術スタックの明確化: dioxus → ratatui + crossterm + fuzzy-matcher
- 設定ファイル名の変更: `~/.koi/orgs.toml` → `~/.koi/remotes.toml`

### 2026-02-21: エイリアス名でリモートを管理する仕組みを導入 (v0.3.x, #21-#24)

**変更概要**:
- `koi remote`: エイリアス名システム導入（git remote名のようにユーザーが命名）
  - `koi remote add <org>`: エイリアス名をプロンプトで入力（`--name` で直接指定も可）
  - `koi remote set-url <alias> <org>`: org名変更に対応（`.koi.skills`は変更不要）
  - `koi remote switch` / `active` フィールド: 廃止
  - `koi remote remove`: 参照中エイリアスがある場合に警告表示
- `koi add` (`koi clone`から改名): 全登録リモートを横断検索
  - 表示形式: `skill-name @alias`（30文字左寄せ + @エイリアス）
  - ローカル・グローバル両方のインストール済みを除外
- `koi restore` (`koi clone --restore`から独立): エイリアス→orgを解決してclone
- `koi new`: リモート選択UI追加（`--remote`/`-r` で直接指定も可）
- `config.rs`: `RemoteEntry.org` フィールド追加、`resolve_org()` / `validate_alias()` 関数追加
- エイリアス名バリデーション: 英数字・ハイフン・アンダースコアのみ

### 2026-02-21: エイリアス導入に伴うデータ移行手順 (#24)

`remotes.toml` と `.koi.skills` のフォーマットが変わる。手動で以下を修正する。

**`~/.koi/remotes.toml`**

```toml
# 変更前
active = "itton-claude-skills"
[remotes]
itton-claude-skills = { description = "個人スキルリポジトリ" }

# 変更後（active削除、org明示）
[remotes.personal]
org = "itton-claude-skills"
```

**各プロジェクトの `.koi.skills`**

```toml
# 変更前
[skills]
moli = "itton-claude-skills/moli"

# 変更後（エイリアス名のみ）
[skills]
moli = "personal"
```

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
