# 競艇スクレイピングアプリ

競艇の出走情報をスクレイピングするTauriアプリケーションです。フロントエンドにReact + TypeScript、バックエンドにRustを使用しています。

## 現在の機能

### 📊 データ取得機能
- **競艇日和サイト**からレース統計データを取得
- 指定した日付・レース番号・競艇場の情報を自動取得
- HTMLファイルの自動保存（`bort-html/`ディレクトリ）

### 📈 取得可能なデータ
- **逃げ率**: 1年間・半年間の逃げ率
- **逃がし率**: 1年間・半年間の逃がし率  
- **差され率**: 1年間の差され率
- **捲られ率**: 1年間の捲られ率
- **直近成績**: 直近10レースでの1着回数

### 🖥️ ユーザーインターフェース
- 日付選択
- レース番号選択（1〜12レース）
- 競艇場選択（全24場対応）
- リアルタイム結果表示

## 技術スタック

### フロントエンド
- **React + TypeScript**: UIコンポーネント
- **Tauri API**: Rustバックエンドとの通信
- **Vite**: バンドラー

### バックエンド（Rust）
- **headless_chrome**: ブラウザ自動化とスクレイピング
- **scraper**: HTMLパースとDOM操作
- **Tauri**: アプリケーションフレームワーク
- **serde**: データシリアライゼーション

## 開発コマンド

```bash
# フロントエンド開発
pnpm dev              # 開発サーバー起動
pnpm build           # プロダクションビルド
pnpm preview         # ビルドプレビュー

# Tauriアプリケーション
pnpm tauri dev       # アプリケーション開発モード
pnpm tauri build     # アプリケーションビルド

# テスト
cd src-tauri && cargo test  # Rustテスト実行
```

## アーキテクチャ

### メインコンポーネント
- **lib.rs**: Tauriコマンドの定義
- **headress.rs**: headless_chromeを使用したスクレイピング
- **parse/biyori/flame.rs**: 競艇日和のHTMLパース機能
- **information.ts**: 競艇場情報管理

### データフロー
1. フロントエンドから日付・レース番号・競艇場番号を送信
2. Rustバックエンドがheadless_chromeでサイトアクセス
3. HTMLを取得・パース
4. 統計データを構造化してフロントエンドに返却

## 将来の拡張予定

- 他の競艇サイトへの対応
- 追加の統計情報取得
- データの永続化機能
- レース予想機能の追加

## 開発環境セットアップ

### 推奨IDE
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 必要な依存関係
- Node.js (pnpm使用)
- Rust
- Chrome/Chromium（headless_chrome用）
