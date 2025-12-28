use crate::parse::table::ParsedTableData;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn parse_table(input_data: &str) -> Result<ParsedTableData, String> {
    crate::parse::table::parse_table_data(input_data)
}
