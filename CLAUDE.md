# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 概要

このプロジェクトは、競艇の出走情報をスクレイピングするTauriアプリケーションです。フロントエンドにReact + TypeScript、バックエンドにRustを使用しています。

## 開発コマンド

### フロントエンド
- `pnpm dev` - 開発サーバーの起動
- `pnpm build` - プロダクションビルド（TypeScriptコンパイル + Viteビルド）
- `pnpm preview` - プロダクションビルドのプレビュー

### Tauriアプリケーション
- `pnpm tauri dev` - Tauriアプリケーション開発モード
- `pnpm tauri build` - アプリケーションのビルド

### テスト
- `cargo test` - Rustのテストを実行（`src-tauri/`ディレクトリ内で実行）
- `cargo test --test test_name` - 特定のテストのみ実行

## アーキテクチャ

### フロントエンド構成
- **React + TypeScript**: UIコンポーネント
- **Tauri API**: Rustバックエンドとの通信
- **Vite**: バンドラー

### バックエンド構成（Rust）
- **lib.rs**: Tauriコマンドの定義とメインアプリケーション
- **headress.rs**: headless_chromeを使用したスクレイピング機能
- **parse/**: サイト別のHTMLパース機能
  - `biyori/flame.rs`: 競艇日和サイトのレースデータ解析
  - `official.rs`: 公式サイト用パーサー
- **fetch.rs**: HTTP リクエスト処理（現在コメントアウト）

### 主要な依存関係
- **headless_chrome**: ブラウザ自動化とスクレイピング
- **scraper**: HTMLパースとDOM操作
- **reqwest**: HTTP クライアント
- **serde**: シリアライゼーション

## 重要なTauriコマンド

### `get_biyori_info`
競艇日和サイトから特定のレース情報を取得します。
- パラメータ: `date`, `race_number`, `place_number`
- 戻り値: レース統計データ（逃げ率、差され率、捲られ率等）

### データ構造
`RaceData`構造体に以下の情報を格納:
- 逃げ率（1年間/半年間）
- 逃がし率（1年間/半年間）
- 刺され率、捲られ率
- 直近10レースでの1着回数

## 開発時の注意点

- スクレイピング先のサイト構造変更に注意
- HTMLファイルは`bort-html/`ディレクトリに日付別で保存
- 競艇場の番号は`src/information.ts`で管理
- テストはHTMLファイルに依存するため、事前にサンプルデータが必要

## ファイル構成の特徴

- モジュール式パーサー: サイト別にparse配下で分離
- 段階的データ処理: fetch → parse → format の流れ
- エラーハンドリング: Result型を使用した適切なエラー処理