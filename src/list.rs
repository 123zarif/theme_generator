use crate::structs::{Theme, Themes};
use std::fs;
use tabled::{
    Table, Tabled,
    settings::{Alignment, Style, object::Columns},
};

#[derive(Tabled)]
struct ThemeRow {
    #[tabled(rename = "#")]
    index: usize,

    #[tabled(inline)]
    theme: Theme,
}
pub enum FunctionType {
    Value(Themes),
    Json(String),
    Format(Table),
}

pub fn list(json: bool, value: bool) -> FunctionType {
    let home = std::env::var("HOME").unwrap();

    let config_path = format!("{}/.config/colorSchemes/themes.json", home);

    let json_file =
        fs::read_to_string(&config_path).expect("Invalid file! File doesnt exist maybe.");
    let themes: Themes = serde_json::from_str(&json_file).unwrap();

    if json {
        println!("{:?}", json_file);

        return FunctionType::Json(json_file);
    }

    let rows: Vec<ThemeRow> = themes
        .themes
        .iter()
        .enumerate()
        .map(|(index, theme)| ThemeRow {
            index: index,
            theme: theme.clone(),
        })
        .collect();

    let mut table = Table::new(&rows);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());

    println!("{}", table);

    if value {
        return FunctionType::Value(themes);
    } else {
        return FunctionType::Format(table);
    }
}
