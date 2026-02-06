# koi - Claude Code スキルパッケージマネージャー

## プロジェクト概要

koiは、Claude Codeのスキルを自分用のリモートリポジトリと接続し、スキルをclone、pull、pushできるCLIツールです。

### コンセプト

- 自分用のClaude Codeスキルを育てるツール
- Neovimプラグインを自分でカスタマイズするように、スキルも自分で成長させる
- pull（取得・更新）もpush（公開）も**同一ユーザー**が行う双方向の管理

### 他のパッケージマネージャーとの違い

| ツール | 関係性 | コマンド |
|--------|--------|----------|
| npm/cargo | 作者（publish） ← 利用者（install/update） | 一方向 |
| koi | 同一人物が pull/push を行う | 双方向 |

この特徴から、gitのpull/pushとの統一感を重視したコマンド設計になっています。

## 技術スタック

- **言語**: Rust
- **CLI**: clap
- **対話的UI**: dioxus（曖昧検索、プロンプト、進行状況表示）
- **Git操作**: git2-rs または gitコマンドのラッパー

## コマンド一覧

### koi clone
スキルをリモートリポジトリから取得します。

```bash
koi clone              # 曖昧検索で選択してclone
koi clone <skill-name> # 直接指定
koi clone -g           # グローバル（~/.claude/skills）にインストール
```

**動作**:
1. リモートリポジトリのスキル一覧を取得
2. dioxusの曖昧検索UIで選択
3. `.claude/skills/` または `~/.claude/skills/` にgit clone

**要件**:
- プライベートリポジトリに対応する必要がある
- GitHub認証方法の候補：
  - SSH鍵（`~/.ssh/id_rsa`）
  - Personal Access Token (PAT)
  - GitHub CLI (`gh auth`)の認証情報を利用

### koi pull
スキルをリモートリポジトリから更新します。

```bash
koi pull <skill-name>  # 指定したスキルを更新
koi pull --all         # すべてのスキルを更新
```

**動作**:
1. スキルのディレクトリに移動
2. `git pull` を実行

### koi push
ローカルの変更をリモートリポジトリに反映します。

```bash
koi push <skill-name>  # 指定したスキルをpush
koi push --all         # すべての変更をpush
```

**動作**:
1. スキルのディレクトリに移動
2. `git add .` でステージング
3. `git commit` でコミット
4. `git push` でリモートに反映

**課題**:
- **githooksの制約**: メインブランチへのコミット/pushが禁止されている環境への対応
- 対応策の候補：
  - **案A**: 別ブランチ運用（`koi-updates`ブランチにpush → PRを作成）
  - **案B**: `--no-verify`フラグを使用（推奨されない）
  - **案C**: koiディレクトリ（`.claude/skills/`）だけhooksを無効化
  - **案D**: koi専用のgit設定で例外処理

### koi list
インストール済みのスキル一覧を表示します。

```bash
koi list               # ローカルのスキル一覧
koi list -g            # グローバルのスキル一覧
```

**動作**:
1. `.claude/skills/` または `~/.claude/skills/` のディレクトリを走査
2. スキル名、バージョン、説明などを表示

### koi remove
スキルをアンインストールします。

```bash
koi remove             # 曖昧検索で選択してremove
koi remove <skill-name> # 直接指定
```

**動作**:
1. インストール済みスキルをリスト化
2. dioxusの曖昧検索UIで選択
3. 確認プロンプトを表示
4. ディレクトリを削除

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
    │   ├── clone.rs      # スキルの取得
    │   ├── pull.rs       # スキルの更新
    │   ├── push.rs       # スキルの公開
    │   ├── list.rs       # インストール済みスキルの一覧
    │   └── remove.rs     # スキルの削除
    ├── repository/
    │   ├── git.rs        # Git操作のラッパー
    │   ├── remote.rs     # リモートリポジトリ情報の管理
    │   └── config.rs     # リポジトリ設定の読み書き
    ├── skill/
    │   ├── metadata.rs   # スキルのメタデータ（SKILL.md解析）
    │   ├── path.rs       # スキルのパス解決
    │   └── validator.rs  # スキルの妥当性検証
    ├── ui/
    │   ├── fuzzy.rs      # 曖昧検索のインタラクティブUI
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
3. **repository層**: Git操作の抽象化
4. **skill層**: スキル管理ロジック
5. **ui層**: dioxusによる対話的UI
6. **utils層**: 共通ユーティリティ

## 設定ファイル

### ~/.koi/config.toml

```toml
[remote]
# デフォルトのリモートリポジトリ
default = "https://github.com/your-username/claude-skills"

# 複数のリモートを管理可能
[remote.repositories]
personal = "https://github.com/your-username/claude-skills"
work = "git@github.com:company/claude-skills.git"

[auth]
# GitHub認証方法 ("ssh" | "token" | "gh-cli")
method = "ssh"

# Personal Access Token（method = "token"の場合）
# token = "ghp_xxxxxxxxxxxx"

[paths]
# デフォルトのインストールパス
local = ".claude/skills"
global = "~/.claude/skills"
```

## 開発方針

### 設計原則

1. **gitとの統一感**: コマンド名や挙動をgitに合わせる
2. **シンプルさ**: 必要最小限の機能に絞る
3. **対話性**: dioxusによる直感的なUI
4. **柔軟性**: プライベートリポジトリやgithooksへの対応

### 実装の優先順位

1. **Phase 1**: 基本コマンド（clone、list、remove）
2. **Phase 2**: Git操作（pull、push）
3. **Phase 3**: 認証機能（プライベートリポジトリ対応）
4. **Phase 4**: githooks対応と高度な機能

## 未解決の課題

### 1. GitHub認証

プライベートリポジトリをサポートするための認証方法を決定する必要がある。

**候補**:
- SSH鍵を使用（最もシンプル、git2-rsで対応可能）
- GitHub CLIの認証情報を利用（`gh auth token`）
- Personal Access Tokenを設定ファイルに保存

### 2. githooksの制約

メインブランチへのコミット/pushが禁止されている環境でどう動作させるか。

**検討事項**:
- ユーザーのワークフローに合わせた柔軟な設定が必要
- デフォルトは安全側（hooks を尊重）に倒す
- オプションで回避方法を提供

### 3. グローバルインストール

`-g`フラグで`~/.claude/skills`にインストールする機能の優先度と実装方法。

## 参考資料

- [cargo](https://github.com/rust-lang/cargo)
- [mise](https://github.com/jdx/mise)
- [npm](https://github.com/npm/cli)
- [helm](https://github.com/helm/helm)
- [homebrew](https://github.com/Homebrew/brew)
