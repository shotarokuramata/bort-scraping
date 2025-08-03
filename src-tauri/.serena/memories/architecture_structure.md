# アーキテクチャと構造

## ディレクトリ構成

```
bort-scraping/
├── src/                    # フロントエンド (React + TypeScript)
│   ├── App.tsx            # メインアプリケーション
│   ├── information.ts     # 競艇場マッピング
│   └── assets/
├── src-tauri/             # バックエンド (Rust + Tauri)
│   ├── src/
│   │   ├── lib.rs         # Tauriコマンド定義
│   │   ├── headress.rs    # スクレイピング機能
│   │   ├── fetch.rs       # HTTP処理（現在未使用）
│   │   └── parse/         # HTMLパース機能
│   │       ├── biyori/    # 競艇日和サイト専用
│   │       │   ├── flame.rs
│   │       │   ├── table_analyzer.rs
│   │       │   └── win_rate.rs
│   │       └── official.rs
│   ├── bort-html/         # スクレイピング結果保存
│   └── Cargo.toml
├── package.json
└── vite.config.ts
```

## データフロー

1. **フロントエンド → バックエンド**
   - React から Tauri API (`invoke`) でコマンド呼び出し
   - 日付、レース番号、競艇場番号をパラメータとして送信

2. **スクレイピング処理**
   - `headress.rs` で headless_chrome によりブラウザ自動化
   - 競艇日和サイトにアクセス
   - HTMLコンテンツを取得

3. **データ解析**
   - `parse/biyori/flame.rs` でHTML解析
   - 構造化データ（RaceData, OddsData）に変換

4. **バックエンド → フロントエンド**
   - 構造化データをJSONとして返却
   - Reactで受信して表示

## 主要Tauriコマンド

- `get_biyori_info` - レース統計データ取得
- `get_odds_info` - 生HTMLオッズデータ取得
- `get_parsed_odds_info` - パース済みオッズデータ取得

## エラーハンドリング戦略

- Rust側: Result型で成功/失敗を管理
- フロントエンド: try-catchでエラーキャッチ
- ユーザー向けエラーメッセージ表示