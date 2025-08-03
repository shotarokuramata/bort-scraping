# コードスタイルと慣例

## TypeScript/React (フロントエンド)

### 命名規則
- **コンポーネント**: PascalCase (`App`, `RaceData`)
- **変数・関数**: camelCase (`raceData`, `fetchRaceData`)
- **インターface**: PascalCase (`PlayerBasicInfo`, `RaceData`)
- **ファイル**: camelCase拡張子つき (`App.tsx`, `information.ts`)

### TypeScript設定
- strict モード有効
- noUnusedLocals, noUnusedParameters 有効
- JSX: react-jsx
- ES2020 ターゲット

### React パターン
- 関数コンポーネント使用
- useState でステート管理
- async/await でTauri API呼び出し
- インターface でpropsとstate型定義

## Rust (バックエンド)

### 命名規則
- **構造体**: PascalCase (`RaceData`, `PlayerBasicInfo`)
- **関数**: snake_case (`get_biyori_info`, `fetch_shusso_info`)
- **変数**: snake_case (`race_no`, `place_no`)
- **モジュール**: snake_case (`biyori`, `parse`)

### エラーハンドリング
- Result型を使用
- エラーは文字列で返すかBox<dyn std::error::Error>
- match文でエラーハンドリング

### Serde使用
- #[derive(Debug, serde::Serialize)] で構造体に付与
- フロントエンドとの通信で自動シリアライゼーション

### モジュール構成
- 機能別にファイル分割
- サイト別パーサーを分離 (`parse/biyori/`, `parse/official.rs`)