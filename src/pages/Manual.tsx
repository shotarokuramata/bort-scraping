import { useState } from "react";
import { CommandDocCard, CommandDocCardProps } from "../components/parts/CommandDocCard";
import "./Manual.css";

type TabType = "scraping" | "open-api" | "payout" | "utils";

function Manual() {
  const [activeTab, setActiveTab] = useState<TabType>("scraping");

  // スクレイピングツールタブのコマンド
  // Tauri実装: src-tauri/src/commands/scraping.rs, utils.rs, schedule.rs
  // Hook使用: src/hooks/useHelloWorld.ts, useActiveRaces.ts, useRaceData.ts, useOddsData.ts, useBulkData.ts
  // 関連ページ: ScrapingTool.tsx
  const scrapingCommands: CommandDocCardProps[] = [
    {
      title: "接続テスト",
      commandName: "greet",
      functionName: "useHelloWorld",
      description: "テスト用のHelloメッセージを取得します。接続確認に使用できます。",
      parameters: [
        {
          name: "name",
          type: "string",
          description: "挨拶する対象の名前",
          required: true,
        },
      ],
      returnType: "string",
      example: `const message = await invoke<string>("greet", { name: "World" });
// => "Hello, World! You've been greeted from Rust!"`,
      implementation: "src-tauri/src/commands/utils.rs:1-4",
    },
    {
      title: "開催中レース場の情報取得",
      commandName: "get_all_venues_with_status",
      functionName: "useActiveRaces",
      description: "現在開催中のレース場情報を取得します。各競艇場の開催状況とレース番号が含まれます。",
      parameters: [],
      returnType: "AllVenuesResponse",
      example: `const venues = await invoke<AllVenuesResponse>("get_all_venues_with_status");
// venues.桐生.status === "active"
// venues.桐生.race_number === 12`,
      implementation: "src-tauri/src/commands/schedule.rs:15-18",
    },
    {
      title: "レース情報の取得",
      commandName: "get_biyori_info",
      functionName: "useRaceData",
      description: "競艇日和サイトから特定レースのデータを取得します。選手ごとの逃げ率、差され率、捲られ率などの統計情報が含まれます。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式、例: 20241225）",
          required: true,
        },
        {
          name: "race_number",
          type: "string",
          description: "レース番号（1-12）",
          required: true,
        },
        {
          name: "place_number",
          type: "string",
          description: "競艇場番号（01-24）",
          required: true,
        },
      ],
      returnType: "RaceData",
      example: `const raceData = await invoke<RaceData>("get_biyori_info", {
  date: "20241225",
  raceNumber: "12",
  placeNumber: "01",
});`,
      implementation: "src-tauri/src/commands/scraping.rs:4-25",
    },
    {
      title: "単勝・複勝オッズの取得",
      commandName: "get_win_place_odds_info",
      functionName: "useOddsData",
      description: "単勝・複勝オッズ情報を取得します。各艇のオッズと組み合わせが含まれます。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "race_number",
          type: "string",
          description: "レース番号（1-12）",
          required: true,
        },
        {
          name: "place_number",
          type: "string",
          description: "競艇場番号（01-24）",
          required: true,
        },
      ],
      returnType: "OddsData",
      example: `const oddsData = await invoke<OddsData>("get_win_place_odds_info", {
  date: "20241225",
  raceNumber: "12",
  placeNumber: "01",
});`,
      implementation: "src-tauri/src/commands/scraping.rs:37-58",
    },
    {
      title: "データの一括取得",
      commandName: "get_bulk_race_data",
      functionName: "useBulkData",
      description: "期間・会場・レース番号を指定して一括でデータを取得します。進捗状況は bulk-progress イベントで通知されます。",
      parameters: [
        {
          name: "start_date",
          type: "string",
          description: "開始日（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "end_date",
          type: "string",
          description: "終了日（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "place_numbers",
          type: "number[]",
          description: "競艇場番号の配列（例: [1, 2, 3]）",
          required: true,
        },
        {
          name: "race_numbers",
          type: "number[]",
          description: "レース番号の配列（例: [1, 2, 12]）",
          required: true,
        },
      ],
      returnType: "BulkRaceData[]",
      example: `// プログレスイベントのリスニング
await listen<BulkProgressPayload>("bulk-progress", (event) => {
  console.log(\`進捗: \${event.payload.current}/\${event.payload.total}\`);
});

// 一括取得実行
const results = await invoke<BulkRaceData[]>("get_bulk_race_data", {
  startDate: "20241201",
  endDate: "20241210",
  placeNumbers: [1, 2, 3],
  raceNumbers: [12],
});`,
      implementation: "src-tauri/src/commands/scraping.rs:60-75",
    },
  ];

  // Open API データ管理タブのコマンド
  // Tauri実装: src-tauri/src/commands/open_api.rs
  // Hook使用: src/hooks/useOpenApi.ts
  // 関連ページ: OpenApiTool.tsx
  const openApiCommands: CommandDocCardProps[] = [
    {
      title: "Open APIサービスの初期化",
      commandName: "init_open_api_service",
      description: "Open APIサービスを初期化します。アプリケーション起動時に自動実行されます。",
      parameters: [],
      returnType: "string",
      example: `const result = await invoke<string>("init_open_api_service");
// => "Open API service initialized successfully"`,
      implementation: "src-tauri/src/commands/open_api.rs:14-25",
    },
    {
      title: "出走表データの取得",
      commandName: "fetch_previews_data",
      functionName: "useOpenApi.fetchData",
      description: "特定日のPreviewsデータ（出走表情報）を取得してデータベースに保存します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
      ],
      returnType: "string",
      example: `const result = await invoke<string>("fetch_previews_data", {
  date: "20241225",
});`,
      implementation: "src-tauri/src/commands/open_api.rs:28-44",
    },
    {
      title: "レース結果データの取得",
      commandName: "fetch_results_data",
      functionName: "useOpenApi.fetchData",
      description: "特定日のResultsデータ（レース結果情報）を取得してデータベースに保存します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
      ],
      returnType: "string",
      example: `const result = await invoke<string>("fetch_results_data", {
  date: "20241225",
});`,
      implementation: "src-tauri/src/commands/open_api.rs:47-63",
    },
    {
      title: "番組情報データの取得",
      commandName: "fetch_programs_data",
      functionName: "useOpenApi.fetchData",
      description: "特定日のProgramsデータ（番組情報）を取得してデータベースに保存します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
      ],
      returnType: "string",
      example: `const result = await invoke<string>("fetch_programs_data", {
  date: "20241225",
});`,
      implementation: "src-tauri/src/commands/open_api.rs:66-82",
    },
    {
      title: "出走表データの一括取得",
      commandName: "fetch_previews_data_bulk",
      functionName: "useOpenApi.fetchDataBulk",
      description: "期間指定でPreviewsデータを一括取得します。進捗は open-api-bulk-progress イベントで通知されます。",
      parameters: [
        {
          name: "start_date",
          type: "string",
          description: "開始日（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "end_date",
          type: "string",
          description: "終了日（YYYYMMDD形式）",
          required: true,
        },
      ],
      returnType: "BulkFetchSummary",
      example: `const summary = await invoke<BulkFetchSummary>("fetch_previews_data_bulk", {
  startDate: "20241201",
  endDate: "20241210",
});`,
      implementation: "src-tauri/src/commands/open_api.rs:376-407",
    },
    {
      title: "レース結果データの一括取得",
      commandName: "fetch_results_data_bulk",
      functionName: "useOpenApi.fetchDataBulk",
      description: "期間指定でResultsデータを一括取得します。進捗は open-api-bulk-progress イベントで通知されます。",
      parameters: [
        {
          name: "start_date",
          type: "string",
          description: "開始日（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "end_date",
          type: "string",
          description: "終了日（YYYYMMDD形式）",
          required: true,
        },
      ],
      returnType: "BulkFetchSummary",
      example: `const summary = await invoke<BulkFetchSummary>("fetch_results_data_bulk", {
  startDate: "20241201",
  endDate: "20241210",
});`,
      implementation: "src-tauri/src/commands/open_api.rs:410-441",
    },
    {
      title: "番組情報データの一括取得",
      commandName: "fetch_programs_data_bulk",
      functionName: "useOpenApi.fetchDataBulk",
      description: "期間指定でProgramsデータを一括取得します。進捗は open-api-bulk-progress イベントで通知されます。",
      parameters: [
        {
          name: "start_date",
          type: "string",
          description: "開始日（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "end_date",
          type: "string",
          description: "終了日（YYYYMMDD形式）",
          required: true,
        },
      ],
      returnType: "BulkFetchSummary",
      example: `const summary = await invoke<BulkFetchSummary>("fetch_programs_data_bulk", {
  startDate: "20241201",
  endDate: "20241210",
});`,
      implementation: "src-tauri/src/commands/open_api.rs:444-475",
    },
    {
      title: "データ取得状況の確認",
      commandName: "get_open_api_data_summary",
      functionName: "useOpenApi.fetchDataSummary",
      description: "日付ごとのデータ取得状況サマリーを取得します。どのデータ種別が取得済みかを確認できます。",
      parameters: [],
      returnType: "DataSummaryRow[]",
      example: `const summary = await invoke<DataSummaryRow[]>("get_open_api_data_summary");
// summary[0] => { date: "20241225", previews: 1, results: 1, programs: 0 }`,
      implementation: "src-tauri/src/commands/open_api.rs:361-371",
    },
    {
      title: "CSV形式でエクスポート",
      commandName: "export_open_api_to_csv_v3",
      functionName: "useOpenApi.exportToCsv",
      description: "正規化スキーマ形式でCSV出力します。races.csv と race_participants.csv の2ファイルが生成されます。",
      parameters: [
        {
          name: "output_dir",
          type: "string",
          description: "出力先ディレクトリパス",
          required: true,
        },
      ],
      returnType: "(number, number)",
      example: `const [raceCount, participantCount] = await invoke<[number, number]>(
  "export_open_api_to_csv_v3",
  { outputDir: "/path/to/output" }
);`,
      implementation: "src-tauri/src/commands/open_api.rs:168-189",
    },
  ];

  // 高配当レース検索タブのコマンド
  // Tauri実装: src-tauri/src/commands/open_api.rs
  // Hook使用: src/hooks/useOpenApi.ts
  // 関連ページ: HighPayoutSearch.tsx
  const payoutCommands: CommandDocCardProps[] = [
    {
      title: "配当統計情報の取得",
      commandName: "get_payout_statistics",
      functionName: "useOpenApi.getPayoutStatistics",
      description: "配当統計情報を取得します。最高配当・平均配当などの統計データが含まれます。",
      parameters: [],
      returnType: "PayoutStats",
      example: `const stats = await invoke<PayoutStats>("get_payout_statistics");
// stats.max_trifecta_payout => 最高3連単配当`,
      implementation: "src-tauri/src/commands/open_api.rs:223-233",
    },
    {
      title: "高配当レースの検索",
      commandName: "search_high_payout_races",
      functionName: "useOpenApi.searchHighPayoutRaces",
      description: "最小配当額と配当種別を指定して高配当レースを検索します。",
      parameters: [
        {
          name: "min_payout",
          type: "number",
          description: "最小配当額（円）",
          required: true,
        },
        {
          name: "payout_type",
          type: "string",
          description: "配当種別（win, place, exacta, trifecta）",
          required: true,
        },
        {
          name: "limit",
          type: "number",
          description: "取得件数の上限（デフォルト: 100）",
          required: false,
        },
      ],
      returnType: "RaceResult[]",
      example: `const results = await invoke<RaceResult[]>("search_high_payout_races", {
  minPayout: 100000,
  payoutType: "trifecta",
  limit: 50,
});`,
      implementation: "src-tauri/src/commands/open_api.rs:194-220",
    },
    {
      title: "詳細条件でレース検索",
      commandName: "search_races_advanced",
      functionName: "useOpenApi.searchAdvanced",
      description: "複合条件で詳細検索を実行します。日付、会場、配当、選手情報など複数の条件を組み合わせて検索できます。",
      parameters: [
        {
          name: "params",
          type: "SearchParams",
          description: "検索パラメータオブジェクト（date_from, date_to, venue_code, min_payout, max_payout, payout_type, racer_number, racer_name, racer_class, limit等）",
          required: true,
        },
      ],
      returnType: "(RaceRecord, RaceParticipantRecord[])[]",
      example: `const results = await invoke("search_races_advanced", {
  params: {
    dateFrom: "20241201",
    dateTo: "20241210",
    minPayout: 50000,
    payoutType: "trifecta",
    limit: 100,
  },
});`,
      implementation: "src-tauri/src/commands/open_api.rs:238-249",
    },
    {
      title: "選手番号でレース検索",
      commandName: "search_races_by_racer",
      description: "選手番号で検索します。特定の選手が出場したレースを検索できます。",
      parameters: [
        {
          name: "racer_number",
          type: "number",
          description: "選手登録番号",
          required: true,
        },
        {
          name: "limit",
          type: "number",
          description: "取得件数の上限（デフォルト: 100）",
          required: false,
        },
      ],
      returnType: "(RaceRecord, RaceParticipantRecord[])[]",
      example: `const results = await invoke("search_races_by_racer", {
  racerNumber: 4444,
  limit: 50,
});`,
      implementation: "src-tauri/src/commands/open_api.rs:252-268",
    },
    {
      title: "選手名でレース検索",
      commandName: "search_races_by_racer_name",
      description: "選手名で部分一致検索します。名前の一部でも検索可能です。",
      parameters: [
        {
          name: "racer_name",
          type: "string",
          description: "選手名（部分一致）",
          required: true,
        },
        {
          name: "limit",
          type: "number",
          description: "取得件数の上限（デフォルト: 100）",
          required: false,
        },
      ],
      returnType: "(RaceRecord, RaceParticipantRecord[])[]",
      example: `const results = await invoke("search_races_by_racer_name", {
  racerName: "田中",
  limit: 50,
});`,
      implementation: "src-tauri/src/commands/open_api.rs:271-287",
    },
    {
      title: "級別でレース検索",
      commandName: "search_races_by_class",
      description: "級別で検索します。A1, A2, B1, B2級の選手が出場したレースを検索できます。",
      parameters: [
        {
          name: "racer_class",
          type: "number",
          description: "級別（1=A1, 2=A2, 3=B1, 4=B2）",
          required: true,
        },
        {
          name: "limit",
          type: "number",
          description: "取得件数の上限（デフォルト: 100）",
          required: false,
        },
      ],
      returnType: "(RaceRecord, RaceParticipantRecord[])[]",
      example: `const results = await invoke("search_races_by_class", {
  racerClass: 1, // A1級
  limit: 50,
});`,
      implementation: "src-tauri/src/commands/open_api.rs:290-307",
    },
    {
      title: "日付範囲でレース検索",
      commandName: "search_races_by_date_range",
      description: "日付範囲で検索します。指定期間のレース結果を取得できます。",
      parameters: [
        {
          name: "date_from",
          type: "string",
          description: "開始日（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "date_to",
          type: "string",
          description: "終了日（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "limit",
          type: "number",
          description: "取得件数の上限（デフォルト: 100）",
          required: false,
        },
      ],
      returnType: "(RaceRecord, RaceParticipantRecord[])[]",
      example: `const results = await invoke("search_races_by_date_range", {
  dateFrom: "20241201",
  dateTo: "20241210",
  limit: 100,
});`,
      implementation: "src-tauri/src/commands/open_api.rs:310-334",
    },
    {
      title: "会場でレース検索",
      commandName: "search_races_by_venue",
      description: "会場コードで検索します。特定の競艇場で開催されたレースを検索できます。",
      parameters: [
        {
          name: "venue_code",
          type: "string",
          description: "会場コード（01-24の2桁形式）",
          required: true,
        },
        {
          name: "limit",
          type: "number",
          description: "取得件数の上限（デフォルト: 100）",
          required: false,
        },
      ],
      returnType: "(RaceRecord, RaceParticipantRecord[])[]",
      example: `const results = await invoke("search_races_by_venue", {
  venueCode: "01", // 桐生
  limit: 50,
});`,
      implementation: "src-tauri/src/commands/open_api.rs:337-358",
    },
  ];

  // ユーティリティタブのコマンド
  // Tauri実装: src-tauri/src/commands/storage.rs, schedule.rs
  // Hook使用: 直接invoke呼び出しまたは既存hooks
  const utilsCommands: CommandDocCardProps[] = [
    {
      title: "レースデータの保存",
      commandName: "save_race_data_to_db",
      description: "レースデータをローカルKVストアに保存します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "place_number",
          type: "number",
          description: "競艇場番号（1-24）",
          required: true,
        },
        {
          name: "race_number",
          type: "number",
          description: "レース番号（1-12）",
          required: true,
        },
        {
          name: "race_data",
          type: "RaceData",
          description: "保存するレースデータ",
          required: true,
        },
      ],
      returnType: "void",
      example: `await invoke("save_race_data_to_db", {
  date: "20241225",
  placeNumber: 1,
  raceNumber: 12,
  raceData: { /* RaceData */ },
});`,
      implementation: "src-tauri/src/commands/storage.rs:4-13",
    },
    {
      title: "保存済みレースデータの取得",
      commandName: "get_race_data_from_db",
      description: "ローカルKVストアからレースデータを取得します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "place_number",
          type: "number",
          description: "競艇場番号（1-24）",
          required: true,
        },
        {
          name: "race_number",
          type: "number",
          description: "レース番号（1-12）",
          required: true,
        },
      ],
      returnType: "RaceData | null",
      example: `const raceData = await invoke<RaceData | null>("get_race_data_from_db", {
  date: "20241225",
  placeNumber: 1,
  raceNumber: 12,
});`,
      implementation: "src-tauri/src/commands/storage.rs:15-23",
    },
    {
      title: "オッズデータの保存",
      commandName: "save_odds_data_to_db",
      description: "オッズデータをローカルKVストアに保存します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "place_number",
          type: "number",
          description: "競艇場番号（1-24）",
          required: true,
        },
        {
          name: "race_number",
          type: "number",
          description: "レース番号（1-12）",
          required: true,
        },
        {
          name: "odds_data",
          type: "OddsData",
          description: "保存するオッズデータ",
          required: true,
        },
      ],
      returnType: "void",
      example: `await invoke("save_odds_data_to_db", {
  date: "20241225",
  placeNumber: 1,
  raceNumber: 12,
  oddsData: { /* OddsData */ },
});`,
      implementation: "src-tauri/src/commands/storage.rs:25-34",
    },
    {
      title: "保存済みオッズデータの取得",
      commandName: "get_odds_data_from_db",
      description: "ローカルKVストアからオッズデータを取得します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "place_number",
          type: "number",
          description: "競艇場番号（1-24）",
          required: true,
        },
        {
          name: "race_number",
          type: "number",
          description: "レース番号（1-12）",
          required: true,
        },
      ],
      returnType: "OddsData | null",
      example: `const oddsData = await invoke<OddsData | null>("get_odds_data_from_db", {
  date: "20241225",
  placeNumber: 1,
  raceNumber: 12,
});`,
      implementation: "src-tauri/src/commands/storage.rs:36-44",
    },
    {
      title: "保存済みデータ一覧の取得",
      commandName: "get_all_stored_race_keys",
      description: "保存済みレースのキー一覧を取得します。どのレースデータが保存されているか確認できます。",
      parameters: [],
      returnType: "string[]",
      example: `const keys = await invoke<string[]>("get_all_stored_race_keys");
// keys => ["20241225_01_12", "20241224_02_10", ...]`,
      implementation: "src-tauri/src/commands/storage.rs:46-50",
    },
    {
      title: "レースデータの削除",
      commandName: "delete_race_data_from_db",
      description: "特定のレースデータを削除します。",
      parameters: [
        {
          name: "date",
          type: "string",
          description: "日付（YYYYMMDD形式）",
          required: true,
        },
        {
          name: "place_number",
          type: "number",
          description: "競艇場番号（1-24）",
          required: true,
        },
        {
          name: "race_number",
          type: "number",
          description: "レース番号（1-12）",
          required: true,
        },
      ],
      returnType: "void",
      example: `await invoke("delete_race_data_from_db", {
  date: "20241225",
  placeNumber: 1,
  raceNumber: 12,
});`,
      implementation: "src-tauri/src/commands/storage.rs:52-56",
    },
    {
      title: "全データの削除",
      commandName: "clear_all_stored_data",
      description: "ローカルKVストアの全データを削除します。注意: 復元できません。",
      parameters: [],
      returnType: "void",
      example: `await invoke("clear_all_stored_data");`,
      implementation: "src-tauri/src/commands/storage.rs:58-62",
    },
    {
      title: "月間スケジュールの取得",
      commandName: "get_monthly_schedule",
      description: "月間スケジュールを取得します。各競艇場の開催予定が確認できます。",
      parameters: [],
      returnType: "MonthlySchedule",
      example: `const schedule = await invoke<MonthlySchedule>("get_monthly_schedule");`,
      implementation: "src-tauri/src/commands/schedule.rs:5-8",
    },
    {
      title: "開催中レース情報の取得",
      commandName: "get_active_races",
      description: "現在開催中のレース情報を取得します。",
      parameters: [],
      returnType: "ActiveRace",
      example: `const activeRaces = await invoke<ActiveRace>("get_active_races");`,
      implementation: "src-tauri/src/commands/schedule.rs:10-13",
    },
  ];

  const renderTabContent = () => {
    switch (activeTab) {
      case "scraping":
        return (
          <div className="tab-panel">
            {scrapingCommands.map((cmd) => (
              <CommandDocCard key={cmd.commandName} {...cmd} />
            ))}
          </div>
        );
      case "open-api":
        return (
          <div className="tab-panel">
            {openApiCommands.map((cmd) => (
              <CommandDocCard key={cmd.commandName} {...cmd} />
            ))}
          </div>
        );
      case "payout":
        return (
          <div className="tab-panel">
            {payoutCommands.map((cmd) => (
              <CommandDocCard key={cmd.commandName} {...cmd} />
            ))}
          </div>
        );
      case "utils":
        return (
          <div className="tab-panel">
            {utilsCommands.map((cmd) => (
              <CommandDocCard key={cmd.commandName} {...cmd} />
            ))}
          </div>
        );
    }
  };

  return (
    <div className="manual-container">
      <div className="manual-content">
        <div className="manual-header">
          <h1>使い方マニュアル</h1>
          <p>各機能の使い方とTauriコマンドのリファレンス</p>
        </div>

        <div className="manual-tabs">
          <button
            className={`tab-button ${activeTab === "scraping" ? "active" : ""}`}
            onClick={() => setActiveTab("scraping")}
          >
            スクレイピングツール
          </button>
          <button
            className={`tab-button ${activeTab === "open-api" ? "active" : ""}`}
            onClick={() => setActiveTab("open-api")}
          >
            Open API データ管理
          </button>
          <button
            className={`tab-button ${activeTab === "payout" ? "active" : ""}`}
            onClick={() => setActiveTab("payout")}
          >
            高配当レース検索
          </button>
          <button
            className={`tab-button ${activeTab === "utils" ? "active" : ""}`}
            onClick={() => setActiveTab("utils")}
          >
            ユーティリティ
          </button>
        </div>

        <div className="tab-content">{renderTabContent()}</div>
      </div>
    </div>
  );
}

export default Manual;
