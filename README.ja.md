# tabletrace

PostgreSQLの変更をリアルタイムで監視するCLIツール。

[![npm version](https://badge.fury.io/js/%40monorka%2Ftabletrace.svg)](https://www.npmjs.com/package/@monorka/tabletrace)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](./README.md) | **日本語**

## 特徴

- 🔍 **リアルタイム監視** - INSERT、UPDATE、DELETE操作をリアルタイムで検知
- 📊 **複数テーブル対応** - 複数のテーブルを同時に監視
- 🎨 **カラー表示** - INSERT(緑)、UPDATE(黄)、DELETE(赤)で色分け
- 🔄 **インタラクティブモード** - 変更の詳細確認、履歴表示、テーブル切り替えが可能
- ⚡ **軽量** - トリガー不要、スキーマ変更不要、パフォーマンスへの影響最小限

> 💡 **Note**: このCLIはローカル開発向けの**開発ツール**です。GUI版は [TableTrace OSS](https://github.com/monorka/tabletrace-oss) を参照。チーム開発やステージング環境向けは TableTrace Pro（近日公開）。

## インストール

### npm（推奨）

```bash
npm install -g @monorka/tabletrace
```

または npx で直接実行：

```bash
npx @monorka/tabletrace watch --preset postgres
```

### Cargo（Rust）

```bash
cargo install tabletrace
```

### 手動ダウンロード

[GitHub Releases](https://github.com/monorka/tabletrace-cli/releases) からお使いのプラットフォーム用のバイナリをダウンロード。

## クイックスタート

### プリセットを使用（ローカル開発におすすめ）

```bash
# ローカルのPostgreSQL（localhost:5432）
tabletrace watch --preset postgres

# Supabase ローカル（localhost:54322）
tabletrace watch --preset supabase
```

### カスタム接続

```bash
# 基本的な接続
tabletrace watch -d mydb -u postgres -W mypassword

# すべてのオプションを指定
tabletrace watch -H localhost -P 5432 -d mydb -u postgres -W mypassword
```

### 環境変数を使用（セキュリティ上推奨）

```bash
export PGPASSWORD=mypassword
tabletrace watch -d mydb -u postgres
```

## 使い方

```
tabletrace watch [オプション]

オプション:
      --preset <PRESET>      プリセット: 'supabase' (localhost:54322) または 'postgres' (localhost:5432)
  -H, --host <HOST>          データベースホスト [デフォルト: localhost]
  -P, --port <PORT>          データベースポート [デフォルト: 5432]
  -d, --database <DATABASE>  データベース名 (--preset未使用時は必須)
  -u, --user <USER>          データベースユーザー [デフォルト: postgres]
  -W, --password <PASSWORD>  データベースパスワード (または PGPASSWORD 環境変数を使用)
  -s, --schema <SCHEMA>      監視するスキーマ ('all'で全スキーマ) [デフォルト: public]
  -i, --interval <INTERVAL>  ポーリング間隔（ミリ秒） [デフォルト: 1000]
      --interactive          インタラクティブモードを有効化 [デフォルト: true]
  -h, --help                 ヘルプを表示
  -V, --version              バージョンを表示
```

## インタラクティブコマンド

インタラクティブモードで使用できるコマンド：

| キー | コマンド |
|-----|---------|
| `1`, `2`, ... | 変更 #N の詳細を表示 |
| `l` | 記録された変更を一覧表示 |
| `c` | 変更履歴をクリア |
| `w` | 現在監視中のテーブルを表示 |
| `r` | 監視テーブルを再選択 |
| `h` | ヘルプを表示 |
| `q` | 終了 |

## 出力例

```
╔══════════════════════════════════════════════════════════╗
║           TableTrace - Real-time DB Monitor              ║
╚══════════════════════════════════════════════════════════╝

👁 Watching (2 tables)
  [1] public.users
  [2] public.orders

+ #1 [14:23:45] INSERT public.users (1 row)
    + id=123 { name=Alice, email=alice@example.com }

~ #2 [14:23:52] UPDATE public.orders (1 row)
    ~ id=456 { status: pending → completed }

- #3 [14:24:01] DELETE public.users (1 row)
    - id=123 { name=Alice, email=alice@example.com }
```

## 仕組み

TableTraceはPostgreSQLの `pg_stat_user_tables` システムビューを監視して変更を検知します：

- ✅ **トリガー不要** - どのPostgreSQLデータベースでも動作
- ✅ **スキーマ変更不要** - 読み取り専用の監視
- ✅ **軽量** - 軽いポーリングを使用
- ✅ **行レベルの差分** - 何が変更されたか詳細に表示

## セキュリティ

- パスワードは `PGPASSWORD` 環境変数で渡すことを推奨
- 接続情報はログに出力されません
- システムカタログとユーザーテーブルからの読み取りのみ

## 動作要件

- PostgreSQL 9.6 以上
- Node.js 16 以上（npmインストールの場合）

## 関連プロジェクト

- [TableTrace OSS](https://github.com/monorka/tabletrace-oss) - ローカル開発向けデスクトップGUIアプリ
- [TableTrace Pro](https://tabletrace.dev) - チーム開発・ステージング対応（近日公開）

## ライセンス

MIT © [Monorka Inc.](https://github.com/monorka)


