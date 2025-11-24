use scraper::{Html, Selector};
#[derive(Debug)]
pub struct TableInfo {
    pub table_index: usize,
    pub row_count: usize,
    pub column_count: usize,
    pub headers: Vec<String>,
    pub sample_rows: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct CellInfo {
    pub table_index: usize,
    pub row_index: usize,
    pub col_index: usize,
    pub content: String,
    pub classes: Vec<String>,
}

pub fn analyze_tables(content: &str) -> Result<Vec<TableInfo>, Box<dyn std::error::Error>> {
    let document = Html::parse_document(content);
    let race_basic_selector = Selector::parse("#raceBasic").unwrap();
    let table_selector = Selector::parse("table.table_fixed").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td, th").unwrap();

    let race_basic = document
        .select(&race_basic_selector)
        .next()
        .ok_or("raceBasic not found")?;

    let tables: Vec<_> = race_basic.select(&table_selector).collect();
    let mut table_infos = Vec::new();

    for (table_index, table) in tables.iter().enumerate() {
        let rows: Vec<_> = table.select(&row_selector).collect();
        let mut headers = Vec::new();
        let mut sample_rows = Vec::new();
        let mut max_columns = 0;

        // 最初の行をヘッダーとして扱う
        if let Some(first_row) = rows.first() {
            let header_cells: Vec<String> = first_row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            max_columns = header_cells.len();
            headers = header_cells;
        }

        // 最初の3行をサンプルとして保存
        for row in rows.iter().take(3) {
            let row_data: Vec<String> = row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            max_columns = max_columns.max(row_data.len());
            sample_rows.push(row_data);
        }

        table_infos.push(TableInfo {
            table_index,
            row_count: rows.len(),
            column_count: max_columns,
            headers,
            sample_rows,
        });
    }

    Ok(table_infos)
}

pub fn find_target_data_location(
    content: &str,
) -> Result<Vec<CellInfo>, Box<dyn std::error::Error>> {
    let document = Html::parse_document(content);
    let race_basic_selector = Selector::parse("#raceBasic").unwrap();
    let table_selector = Selector::parse("table.table_fixed").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td, th").unwrap();

    let race_basic = document
        .select(&race_basic_selector)
        .next()
        .ok_or("raceBasic not found")?;

    let tables: Vec<_> = race_basic.select(&table_selector).collect();
    let mut found_cells = Vec::new();

    // tables[5] (6番目のテーブル) の rows[5] (6番目の行) を詳細調査
    if tables.len() > 5 {
        let table = &tables[5];
        let rows: Vec<_> = table.select(&row_selector).collect();

        println!("=== Table 5 (6番目のテーブル) の構造 ===");
        println!("行数: {}", rows.len());

        for (row_index, row) in rows.iter().enumerate() {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            println!("Row {}: {} columns", row_index, cells.len());

            for (col_index, cell) in cells.iter().enumerate() {
                let content = cell.text().collect::<String>().trim().to_string();
                let classes: Vec<String> = cell
                    .value()
                    .attr("class")
                    .unwrap_or("")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

                if row_index == 5 {
                    found_cells.push(CellInfo {
                        table_index: 5,
                        row_index,
                        col_index,
                        content: content.clone(),
                        classes: classes.clone(),
                    });
                }

                println!(
                    "  Col {}: '{}' (classes: {:?})",
                    col_index, content, classes
                );
            }
            println!();
        }
    }

    Ok(found_cells)
}

pub fn analyze_course_data_structure(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let document = Html::parse_document(content);
    let race_basic_selector = Selector::parse("#raceBasic").unwrap();
    let table_selector = Selector::parse("table.table_fixed").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td, th").unwrap();

    let race_basic = document
        .select(&race_basic_selector)
        .next()
        .ok_or("raceBasic not found")?;

    let tables: Vec<_> = race_basic.select(&table_selector).collect();

    println!("=== 全テーブルの概要 ===");
    for (table_index, table) in tables.iter().enumerate() {
        let rows: Vec<_> = table.select(&row_selector).collect();
        println!("Table {}: {} rows", table_index, rows.len());

        // 最初の行のヘッダーを確認
        if let Some(first_row) = rows.first() {
            let headers: Vec<String> = first_row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            println!("  Headers: {:?}", headers);
        }

        // course1, course2などのクラスを持つセルを探す
        let course_cells: Vec<_> = table
            .select(&Selector::parse("td[class*='course'], th[class*='course']").unwrap())
            .collect();

        if !course_cells.is_empty() {
            println!("  Course-related cells found: {}", course_cells.len());
            for cell in course_cells.iter().take(6) {
                let content = cell.text().collect::<String>().trim().to_string();
                let classes = cell.value().attr("class").unwrap_or("");
                println!("    '{}' (class: {})", content, classes);
            }
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_analyze_tables() {
        let file_path = "./bort-html/20250704/biyori.html";
        let content = fs::read_to_string(file_path).expect("Failed to read HTML file");

        match analyze_tables(&content) {
            Ok(table_infos) => {
                for table_info in table_infos {
                    println!("{:#?}", table_info);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_find_target_data_location() {
        let file_path = "./bort-html/20250704/biyori.html";
        let content = fs::read_to_string(file_path).expect("Failed to read HTML file");

        match find_target_data_location(&content) {
            Ok(cells) => {
                println!("=== Tables[5] Row[5] の内容 ===");
                for cell in cells {
                    println!("{:#?}", cell);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_analyze_course_data_structure() {
        let file_path = "./bort-html/20250704/biyori.html";
        let content = fs::read_to_string(file_path).expect("Failed to read HTML file");

        if let Err(e) = analyze_course_data_structure(&content) {
            eprintln!("Error: {}", e);
        }
    }
}
