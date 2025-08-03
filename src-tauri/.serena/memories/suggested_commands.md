# 推奨コマンド

## 開発コマンド

### フロントエンド開発
```bash
pnpm dev          # 開発サーバーの起動（Vite）
pnpm build        # プロダクションビルド（TypeScript + Viteビルド）
pnpm preview      # プロダクションビルドのプレビュー
```

### Tauriアプリケーション開発
```bash
pnpm tauri dev    # Tauriアプリケーション開発モード（推奨）
pnpm tauri build  # アプリケーションのビルド
```

### テスト・品質管理
```bash
# Rustのテスト（src-tauri/ディレクトリ内で実行）
cd src-tauri
cargo test        # 全テストを実行
cargo test --test test_name  # 特定のテストのみ実行

# TypeScriptの型チェック
pnpm build        # TypeScriptコンパイルも含む
```

### パッケージ管理
```bash
pnpm install      # 依存関係のインストール
```

## 重要な開発用ファイルパス
- `src-tauri/bort-html/` - スクレイピング結果のHTMLファイル保存場所
- `src/information.ts` - 競艇場番号とマッピング
- `src-tauri/src/lib.rs` - Tauriコマンド定義
- `src-tauri/src/headress.rs` - スクレイピング機能
- `src-tauri/src/parse/biyori/` - 競艇日和サイト専用パーサー