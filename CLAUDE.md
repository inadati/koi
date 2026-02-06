# CLAUDE.md

このファイルは、Claude Codeがkoiリポジトリのコードを扱う際のガイダンスを提供します。

## プロジェクト概要

koiは、Claude Codeのスキルを管理するためのCLIパッケージマネージャーです。

- **言語**: Rust
- **CLI**: clap
- **対話的UI**: dioxus
- **開発フレームワーク**: moli (v2.1.0)

## 開発方針

### moliフレームワークの使用

このプロジェクトは[moli](https://github.com/kyoheiu/moli)フレームワークを使用して開発します。

- `moli.yml`でプロジェクト構造を定義
- `moli up`でコード生成
- 詳細な仕様は`MOLI.md`を参照

### コマンド設計の原則

koiのコマンド設計は、以下の特徴に基づいています：

**他のパッケージマネージャーとの違い**:
- npm/cargo: 作者（publish） ← 利用者（install/update）の**一方向**
- koi: 同一人物がpull/pushを行う**双方向**の管理

この特徴から、**gitとの統一感**を重視したコマンド設計になっています：

```bash
koi clone     # git clone に相当
koi pull      # git pull に相当
koi push      # git push に相当
```

### なぜ install/update ではなく clone/pull/push なのか

- `install`/`update`: npm、cargo、brew風（一方向のパッケージ管理）
- `clone`/`pull`/`push`: git風（双方向の管理）

koiはスキルの**取得と公開の両方を同一ユーザーが行う**ため、gitの命名が最も適切です。

## コマンド一覧

| コマンド | 説明 | 対話的UI |
|---------|------|----------|
| `koi clone` | スキルを取得 | 曖昧検索で選択 |
| `koi pull` | スキルを更新 | - |
| `koi push` | スキルを公開 | - |
| `koi list` | インストール済みスキル一覧 | - |
| `koi remove` | スキルを削除 | 曖昧検索で選択 |

### なぜ search, init, link がないのか

- **search**: `koi clone`が曖昧検索機能を持つため不要
- **init**: `expert-skill-make`など既存のClaude Codeスキルがあるため不要
- **link**: `install`のローカルパス対応で代替可能なため不要

## 既知の課題

### 1. プライベートリポジトリの認証

認証方法の候補：
- SSH鍵（`~/.ssh/id_rsa`）
- Personal Access Token (PAT)
- GitHub CLI (`gh auth`)の認証情報を利用

### 2. githooksの制約

現在、githooksで全環境においてメインブランチへのコミット/pushを禁止しています。
`koi push`コマンドでこれを回避する方法を検討中です。

対応策の候補：
- 別ブランチ運用（`koi-updates`ブランチ → PR作成）
- koiディレクトリ（`.claude/skills/`）のみhooksを無効化
- koi専用のgit設定で例外処理

## 開発履歴

### 2026-02-06: プロジェクト初期設計

- moli.ymlの設計と作成
- コマンド構成の検討（clone/pull/push/list/remove）
- MOLI.mdに詳細仕様を記録
- アーキテクチャの設計（6層構成）

**レイヤー構成**:
1. cli層: clapによるCLI定義
2. commands層: サブコマンドの実装
3. repository層: Git操作の抽象化
4. skill層: スキル管理ロジック
5. ui層: dioxusによる対話的UI
6. utils層: 共通ユーティリティ
