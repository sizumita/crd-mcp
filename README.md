# CRD-MCP

## 概要
CRD-MCP は、国立国会図書館が提供するレファレンス協同データベースシステム（CRD）の検索機能を MCP（Model Context Protocol）として公開するための CLI サーバーです。CRD API を代理で呼び出し、自然言語エージェントや MCP 対応クライアントからレファレンス事例・調べ方案内・特別コレクション・参加館プロファイルを横断検索できます。

本MCPは、レファレンス協同データベースのAPI2.0を利用しています。

## 主な機能
- MCP Tool `search` を 1 つ提供し、CRD API の検索条件をそのまま指定可能
- CQL（Contextual Query Language）による柔軟なクエリ記述に対応
- ヒット件数・検索結果セット・エラー情報を構造化 JSON として返却

## 動作要件
- Rust 1.77 以降（edition 2024 を使用）
- `cargo` コマンドが利用可能な環境
- ネットワークから `https://crd.ndl.go.jp/api/refsearch` へアクセスできること

## インストール
```bash
cargo install crd-mcp
```
`stdin/stdout` 経由で通信する MCP サーバーとして起動するため、Claude Desktop など任意の MCP クライアントから「外部サーバー」として登録してください。

```bash
$ claude mcp add crd-mcp -- crd-mcp
```

## ログ
`RUST_LOG` 環境変数でログレベルを制御できます。例: `RUST_LOG=info cargo run --release`。指定がない場合は DEBUG レベルまで標準エラーへ出力します。

## ライセンス

このプロジェクトは MIT ライセンスの下でライセンスされています。
