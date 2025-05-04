// use scraper::{Html, Selector};
// use url::Url;

// pub fn get_race_param_list_from_race_index(html: &str) -> Vec<String> {
//     let document = Html::parse_document(html);

//     let selector = Selector::parse("a[href]").unwrap();
//     let mut query_params_list = Vec::new();

//     for element in document.select(&selector) {
//         if let Some(href) = element.value().attr("href") {
//             // 特定の文字列を含むリンクだけを対象にする
//             if href.contains("/owpc/pc/race/beforeinfo") {
//                 // URLを解析してクエリパラメータを抽出
//                 if let Ok(parsed_url) = Url::parse(&format!("https://www.boatrace.jp{}", href)) {
//                     let query_pairs: Vec<String> = parsed_url
//                         .query_pairs()
//                         .map(|(k, v)| format!("{}={}", k, v))
//                         .collect();
//                     query_params_list.push(query_pairs.join("&"));
//                 }
//             }
//         }
//     }
//     query_params_list
// }

// #[test]
// fn test_get_race_param_list_from_race_index() {
//     use std::fs;
//     let file_path = "./bort-html/20231001/race_index.html";
//     let html_string = fs::read_to_string(file_path).unwrap();
//     let url_list = get_race_param_list_from_race_index(&html_string);
//     println!("{:?}", url_list);
// }
