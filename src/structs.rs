use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Clone, Serialize, Deserialize, Debug, Tabled)]
pub struct Colors {
    #[tabled(display = "display_color_block")]
    pub primary: String,
    #[tabled(display = "display_color_block")]
    pub secondary: String,
    #[tabled(display = "display_color_block")]
    pub light: String,
    #[tabled(display = "display_color_block")]
    pub active: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Tabled)]
pub struct Theme {
    pub name: String,
    pub selected: bool,
    pub quickshell_theme: String,
    #[tabled(inline)]
    pub colors: Colors,
    pub wallpaper: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Themes {
    pub themes: Vec<Theme>,
}

fn display_color_block(hex: &String) -> String {
    let hex_clean = hex.trim_start_matches('#');

    if hex_clean.len() == 6 {
        let r = u8::from_str_radix(&hex_clean[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_clean[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_clean[4..6], 16).unwrap_or(0);

        format!("\x1b[38;2;{};{};{}m██\x1b[0m {}", r, g, b, hex)
    } else {
        hex.to_string()
    }
}
