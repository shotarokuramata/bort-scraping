export interface ParsedTableData {
  line_count: number;
  char_count: number;
  summary: string;
  data: string[];
}

export interface TableWithHeaderAndValues {
  headers: string[];
  rows: string[][];
}
