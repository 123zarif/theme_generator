use serde::Deserialize;
use std::fs;
use tabled::{
    Table, Tabled,
    settings::{Alignment, Style, object::Columns},
};

#[derive(Deserialize, Debug, Tabled)]
struct Colors {
    #[tabled(display = "display_color_block")]
    primary: String,
    #[tabled(display = "display_color_block")]
    secondary: String,
    #[tabled(display = "display_color_block")]
    light: String,
    #[tabled(display = "display_color_block")]
    active: String,
}

#[derive(Deserialize, Debug, Tabled)]
struct Theme {
    name: String,
    selected: bool,
    quickshell_theme: String,
    #[tabled(inline)]
    colors: Colors,
    wallpaper: String,
}

#[derive(Deserialize, Debug)]
struct Themes {
    themes: Vec<Theme>,
}

fn display_color_block(hex: &String) -> String {
    let hex_clean = hex.trim_start_matches('#');

    // Ensure it's a valid 6-character hex string
    if hex_clean.len() == 6 {
        // Parse the hex pairs into u8 RGB integers
        let r = u8::from_str_radix(&hex_clean[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_clean[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_clean[4..6], 16).unwrap_or(0);

        format!("\x1b[38;2;{};{};{}m██\x1b[0m {}", r, g, b, hex)
    } else {
        hex.to_string()
    }
}

pub fn list(json: bool) {
    let home = std::env::var("HOME").unwrap();

    let config_path = format!("{}/.config/colorSchemes/themes.json", home);

    let json_file =
        fs::read_to_string(&config_path).expect("Invalid file! File doesnt exist maybe.");

    if json {
        println!("{:?}", json_file);
        return;
    }

    let themes: Themes = serde_json::from_str(&json_file).unwrap();
    let mut table = Table::new(&themes.themes);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());

    println!("{}", table);
}
