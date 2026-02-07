# koi - Claude Code スキルパッケージマネージャー

## プロジェクト概要

koiは、Claude Codeのスキルをリモートリポジトリ（GitHub）と同期管理するCLIツールです。

### コンセプト

- 自分用のClaude Codeスキルを育てるツール
- Neovimプラグインを自分でカスタマイズするように、スキルも自分で成長させる
- install（取得）とupdate（同期）で直感的に管理
- 内部的にはローカル⇔リモートの双方向同期を行う

## 技術スタック

- **言語**: Rust
- **CLI**: clap
- **対話的UI**: dioxus（曖昧検索、プロンプト、進行状況表示）
- **外部依存**: gh CLI（GitHub CLI）+ git
- **gh CLI**: 認証、GitHub API操作（リポジトリ一覧・作成、org設定）
- **git**: ローカルリポジトリ操作（clone、pull、push、stash）。`-C`オプションでディレクトリ指定。

## コマンド一覧

### koi install (`koi i`)
スキルをリモートリポジトリから取得します。

```bash
koi install              # 曖昧検索で選択してインストール（.koi.skillsに追記）
koi install <skill-name> # 直接指定
koi install -g           # グローバル（~/.claude/skills）にインストール
koi install --restore    # .koi.skillsから一括復元（koi i -r）
```

**動作（通常）**:
1. `gh api` でリモートリポジトリのスキル一覧を取得
2. dioxusの曖昧検索UIで選択
3. `git clone` で `.claude/skills/` にクローン
4. `.koi.skills` にスキル情報を追記

**動作（--restore）**:
1. `.koi.skills` を読み込み
2. 記載された全スキルを `git clone` で `.claude/skills/` にクローン

**認証**: gh CLI（`gh auth login`）に一任。プライベートリポジトリ対応済み。

### koi update (`koi u`)
リモートからローカルへスキルを同期します（pull）。

```bash
koi update       # プロジェクトローカルの全スキルを同期
koi update -g    # グローバルの全スキルを同期
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

### koi remote update (`koi r u`)
ローカルの変更をリモートに反映します（push）。

```bash
koi remote update       # プロジェクトローカルの変更をリモートに反映
koi remote update -g    # グローバルの変更をリモートに反映
```

**動作**:
1. 全インストール済みスキルに対して（`git -C` でディレクトリ指定）:
   a. `git add .` + `git commit --no-verify -m "update skill"`
   b. `git push --no-verify`
   c. pushが失敗した場合（リモートが先行）→ エラーを出し `koi update` を先に実行するよう案内

**push失敗時の出力**:
```
Error: スキル "moli" のリモートが別の環境で更新されています
  先に koi update を実行してください
```

### koi remote set-org
スキルの検索・作成対象となるGitHub orgを設定します。

```bash
koi remote set-org <org-name>   # 例: koi remote set-org itton-claude-skills
```

**動作**: `~/.koi/config.toml` の `[remote] org` を更新。

### koi list
インストール済みのスキル一覧を表示します。

```bash
koi list               # ローカルのスキル一覧
koi list -g            # グローバルのスキル一覧
```

**動作**:
1. `.claude/skills/` または `~/.claude/skills/` のディレクトリを走査
2. スキル名、バージョン、説明などを表示

### koi uninstall
スキルをアンインストールします。

```bash
koi uninstall              # 曖昧検索で選択してアンインストール
koi uninstall <skill-name> # 直接指定
koi uninstall -g           # グローバルのスキルを削除
```

**動作**:
1. インストール済みスキルをリスト化
2. dioxusの曖昧検索UIで選択
3. 確認プロンプトを表示
4. ディレクトリを削除
5. `.koi.skills` から該当スキルを削除

### スキル管理ファイル

| ファイル | 用途 |
|---------|------|
| `.koi.skills` | プロジェクトローカルのスキル管理。リポジトリにコミットし `koi i -r` で一括復元可能。 |
| `~/.koi/global.skills` | グローバルスキルのリモートマッピング。update -g / remote update -g 用。 |

両ファイルは同じフォーマット：

```toml
[skills]
moli = "itton-claude-skills/moli"
expert-skill-make = "itton-claude-skills/expert-skill-make"
expert-git-commit = "itton-claude-skills/expert-git-commit"
```

- `koi install` / `koi install -g` 時に自動追記
- `koi uninstall` / `koi uninstall -g` 時に自動削除

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
    │   ├── install.rs    # スキルの取得 / --restoreで一括復元
    │   ├── update.rs     # リモートからローカルへ同期（pull）
    │   ├── new.rs        # 新規スキル作成（リモートリポジトリ + SKILL.md + ローカルDL）
    │   ├── remote.rs     # koi remote update / koi remote set-org
    │   ├── list.rs       # インストール済みスキルの一覧
    │   └── uninstall.rs  # スキルの削除
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
3. **github層**: gh CLI（GitHub API操作）
4. **git層**: gitコマンドのラッパー（clone、pull、push、stash）
5. **skill層**: スキル管理ロジック
6. **ui層**: dioxusによる対話的UI
7. **utils層**: 共通ユーティリティ

## 設定ファイル

### ~/.koi/config.toml

```toml
[remote]
# スキルの検索・作成対象のGitHub org
org = "itton-claude-skills"

[paths]
# デフォルトのインストールパス
local = ".claude/skills"
global = "~/.claude/skills"
```

**認証**: gh CLI（`gh auth login`）に一任するため、設定ファイルに認証情報は持たない。

## 開発方針

### 設計原則

1. **パッケージマネージャーの使用感**: install/update/list/uninstallの馴染みあるコマンド体系
2. **シンプルさ**: 外部依存はgh CLI + gitのみ
3. **対話性**: dioxusによる直感的なUI
4. **プライベート前提**: gh authによる認証でプライベートリポジトリをサポート

### 実装の優先順位

1. **Phase 1**: 基本コマンド（install、list、uninstall）+ gh CLI認証チェック + .koi.skills管理
2. **Phase 2**: 同期コマンド（update、remote update）+ stash機能
3. **Phase 3**: 対話的UI（dioxus曖昧検索）

## 解決済みの課題

### GitHub認証 → gh CLIに一任
- `gh auth login` で認証済みであることを前提とする
- 未認証の場合はインストール・認証を案内

### githooksの制約 → 解消
- `--no-verify` フラグでgithooksをスキップ
- koiの内部処理でのみ使用するため、ユーザーの通常開発には影響しない

## 未解決の課題

（現時点で未解決の課題はなし）

## 参考資料

- [cargo](https://github.com/rust-lang/cargo)
- [mise](https://github.com/jdx/mise)
- [npm](https://github.com/npm/cli)
- [helm](https://github.com/helm/helm)
- [homebrew](https://github.com/Homebrew/brew)
