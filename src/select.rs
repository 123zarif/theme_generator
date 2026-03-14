use std::fs;
use std::process::Command;

use serde::Deserialize;
use serde::Serialize;

use crate::list::FunctionType;
use crate::list::list;

#[derive(Serialize, Deserialize)]
struct Quickshell {
    theme: String,
    primary: String,
    secondary: String,
    light: String,
    active: String,
}

pub fn select(index: Option<usize>) {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/colorSchemes/themes.json", home);
    let wallpaper_path: String = format!("{}/.config/colorSchemes/wallpapers/", home);
    let hyprland_colors_paths = format!("{}/.config/hypr/colors.conf", home);
    let quickshell_config_paths = format!("{}/.config/quickshell/persists/colors.json", home);

    let mut themes = match list(false) {
        FunctionType::Value(th) => th,
        FunctionType::Json => return,
    };

    let sel_index: usize = match index {
        None => get_index(themes.themes.len()),
        Some(ind) => {
            if ind > themes.themes.len() - 1 {
                println!(
                    "Invalid Index Provide. Please provide from (0 - {})",
                    themes.themes.len() - 1
                );
                get_index(themes.themes.len())
            } else {
                ind
            }
        }
    };

    for (index, theme) in themes.themes.iter_mut().enumerate() {
        if index == sel_index {
            theme.selected = true;
        } else {
            theme.selected = false;
        }
    }

    let config_json =
        serde_json::to_string_pretty(&themes).expect("Couldnt convert config file to JSON!");

    let hypr_primary = format!(
        "$primary = rgb({})",
        themes.themes[sel_index].colors.primary.replace("#", "")
    );
    let hypr_secondary = format!(
        "$secondary = rgb({})",
        themes.themes[sel_index].colors.secondary.replace("#", "")
    );
    let hypr_light = format!(
        "$light = rgb({})",
        themes.themes[sel_index].colors.light.replace("#", "")
    );
    let hypr_active = format!(
        "$active = rgb({})",
        themes.themes[sel_index].colors.active.replace("#", "")
    );

    let hyprland_colors = format!(
        "{} \n {} \n {} \n {}",
        hypr_primary, hypr_secondary, hypr_light, hypr_active,
    );

    let quickshell_config = Quickshell {
        theme: themes.themes[sel_index].quickshell_theme.clone(),
        primary: themes.themes[sel_index].colors.primary.clone(),
        secondary: themes.themes[sel_index].colors.secondary.clone(),
        light: themes.themes[sel_index].colors.light.clone(),
        active: themes.themes[sel_index].colors.active.clone(),
    };

    let quickshell_json = serde_json::to_string_pretty(&quickshell_config)
        .expect("Couldnt convert quickshell config to JSON!");

    fs::write(config_path, config_json).expect("Couldnt update config file!");
    fs::write(hyprland_colors_paths, hyprland_colors)
        .expect("Couldnt upload colors to hypland config!");
    fs::write(quickshell_config_paths, quickshell_json)
        .expect("Couldnt upload colors to hypland config!");

    Command::new("hyprctl")
        .arg("dispatch")
        .arg("exec")
        .arg(format!(
            "awww img --transition-fps 144 --transition-step 155 --transition-type random {}{}",
            wallpaper_path, themes.themes[sel_index].wallpaper
        ))
        .output()
        .expect("Failed to change qallpaper!");

    Command::new("sh")
        .arg("-c")
        .arg("qs kill")
        .output()
        .expect("Failed to kill quickshell!");

    Command::new("hyprctl")
        .arg("dispatch")
        .arg("exec")
        .arg("qs")
        .output()
        .expect("Failed to start quickshell!");
}

fn get_index(len: usize) -> usize {
    loop {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Dumb ass!");

        match input.trim().parse() {
            Ok(num) => {
                if num > len - 1 {
                    println!(
                        "Invalid Index Provide. Please provide from (0 - {})",
                        len - 1
                    );
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!(
                    "Invalid Index Provide. Please provide from (0 - {})",
                    len - 1
                );
                continue;
            }
        };
    }
}
