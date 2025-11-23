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
**基本コマンド:**
- `cargo test` - 全テスト実行（時間がかかる）
- `cargo test --lib` - ライブラリテストのみ実行

**高速テスト（開発時推奨、~1秒）:**
- `cargo test tests::test_get_biyori_info_invalid` - パラメータ検証テスト（瞬時）
- `cargo test tests::test_get_win_place_odds_info_invalid` - オッズ検証テスト（瞬時）
- `cargo test tests::test_get_bulk_race_data_invalid` - 一括取得エラーテスト（瞬時）
- `cargo test parse::biyori::flame::tests` - HTMLパース機能テスト（0.1秒）

**実データテスト（5~30秒、スクレイピング実行）:**
- `cargo test tests::test_get_biyori_info_valid` - 実データ取得テスト（~10秒）
- `cargo test tests::test_get_win_place_odds_info_valid` - 実オッズ取得テスト（~8秒）
- `cargo test tests::test_get_bulk_race_data_valid` - 実一括取得テスト（~15秒）
- `cargo test headress::tests` - スクレイピング実行テスト（~15秒）

**カテゴリ別テスト:**
- `cargo test tests::` - Tauriコマンド関数テスト
- `cargo test parse::` - HTMLパース機能テスト
- `cargo test headress::` - スクレイピング機能テスト

**推奨ワークフロー:**
```bash
# 1. 開発中の高速チェック（~1秒）
cargo test tests::test_get_biyori_info_invalid

# 2. リリース前の完全テスト（~1分）
cargo test --lib

# 3. HTML/スクレイピング確認（~10秒）
cargo test headress::tests::test_fetch_win_place_odds_from_kyoteibiyori -- --nocapture
```

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

## 実装方針

### オッズ機能の実装方針
**2連単・3連単オッズ機能を実装します。**
- 単勝・複勝に加えて2連単・3連単をサポート
- スクレイピング方法: 静的サイトからHTTP GETで取得（headless_chrome不要）
- fetcher.rsモジュールでreqwestを使用してHTMLを取得
- 以前の実装では3連単データの不安定性が課題だったが、静的サイトからの取得とロジック整理で再実装

**実装アプローチ**:
1. 静的HTMLを提供するオッズページのURLを特定（競艇公式サイトまたは競艇日和）
2. reqwestでHTTP GETリクエストを送信してHTMLを取得
3. scraperクレートでHTMLをパースしてオッズデータを抽出
4. 既存のOddsData/OddsCombination構造体を活用（型定義は完成済み）

## ファイル構成の特徴

- モジュール式パーサー: サイト別にparse配下で分離
- 段階的データ処理: fetch → parse → format の流れ
- エラーハンドリング: Result型を使用した適切なエラー処理