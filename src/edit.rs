use std::fs;
use std::path::Path;

use crate::list::list;
use crate::select::select;
use crate::{list::FunctionType, structs::Themes};

pub fn edit(index: Option<usize>) {
    let home = std::env::var("HOME").unwrap();
    let config_path = format!("{}/.config/colorSchemes/themes.json", home);

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

    println!("Selected {}", sel_index);
    println!("1. Name: {}", themes.themes[sel_index].name);
    println!(
        "2. Primary Color: {}",
        themes.themes[sel_index].colors.primary
    );
    println!(
        "3. Secondary Color: {}",
        themes.themes[sel_index].colors.secondary
    );
    println!("4. Light Color: {}", themes.themes[sel_index].colors.light);
    println!(
        "5. Active Color: {}",
        themes.themes[sel_index].colors.active
    );

    let mut input: String = String::new();

    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!");
        if input.trim() == "1" {
            edit_name(&mut themes, sel_index);
            break;
        } else if input.trim() == "2" {
            edit_primary(&mut themes, sel_index);
            break;
        } else if input.trim() == "3" {
            edit_secondary(&mut themes, sel_index);
            break;
        } else if input.trim() == "4" {
            edit_light(&mut themes, sel_index);
            break;
        } else if input.trim() == "5" {
            edit_active(&mut themes, sel_index);
            break;
        } else {
            println!("Wrong input!");
            continue;
        }
    }

    if themes.themes[sel_index].selected {
        select(Some(sel_index));
    }

    let config_json =
        serde_json::to_string_pretty(&themes).expect("Couldnt convert config file to JSON!");

    fs::write(config_path, config_json).expect("Couldnt update config file!");
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

fn edit_name(themes: &mut Themes, sel_index: usize) {
    println!("Current Name: {}", themes.themes[sel_index].name);
    println!("Enter New Name:");

    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong!");

    themes.themes[sel_index].name = input.trim().to_string();

    let wallpaper_path: String = format!(
        "{}/.config/colorSchemes/wallpapers/{}",
        std::env::var("HOME").unwrap(),
        themes.themes[sel_index].wallpaper
    );

    let file_name = input
        .trim()
        .to_lowercase()
        .replace(" ", "_")
        .replace(".", "_");

    let ext = Path::new(&wallpaper_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    let full_file_name = format!("{}.{}", file_name.trim(), ext.trim());

    let new_wallpaper_path: String = format!(
        "{}/.config/colorSchemes/wallpapers/{}",
        std::env::var("HOME").unwrap(),
        &full_file_name
    );

    fs::rename(wallpaper_path, &new_wallpaper_path).expect("Couldnt rename wallpaper!");
    themes.themes[sel_index].wallpaper = full_file_name;

    println!("Updated Name!");
}

fn edit_primary(themes: &mut Themes, sel_index: usize) {
    println!("Name: {}", themes.themes[sel_index].name);
    println!(
        "Current Primary Color: {}",
        themes.themes[sel_index].colors.primary
    );
    println!("Enter Primary Color Hex:");

    let mut input: String = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!");

        let trimmed = input.trim();

        let is_hex = trimmed.starts_with('#')
            && trimmed.len() == 7
            && trimmed[1..].chars().all(|c| c.is_ascii_hexdigit());

        if is_hex {
            themes.themes[sel_index].colors.primary = trimmed.to_string();
            break;
        } else {
            println!("Invalid Hex!");
            continue;
        }
    }

    println!("Updated Primary Color!");
}

fn edit_secondary(themes: &mut Themes, sel_index: usize) {
    println!("Name: {}", themes.themes[sel_index].name);
    println!(
        "Current Secondary Color: {}",
        themes.themes[sel_index].colors.secondary
    );
    println!("Enter Secondary Color Hex:");

    let mut input: String = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!");

        let trimmed = input.trim();

        let is_hex = trimmed.starts_with('#')
            && trimmed.len() == 7
            && trimmed[1..].chars().all(|c| c.is_ascii_hexdigit());

        if is_hex {
            themes.themes[sel_index].colors.secondary = trimmed.to_string();
            break;
        } else {
            println!("Invalid Hex!");
            continue;
        }
    }

    println!("Updated Secondary Color!");
}

fn edit_light(themes: &mut Themes, sel_index: usize) {
    println!("Name: {}", themes.themes[sel_index].name);
    println!(
        "Current Light Color: {}",
        themes.themes[sel_index].colors.secondary
    );
    println!("Enter Light Color Hex:");

    let mut input: String = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!");

        let trimmed = input.trim();

        let is_hex = trimmed.starts_with('#')
            && trimmed.len() == 7
            && trimmed[1..].chars().all(|c| c.is_ascii_hexdigit());

        if is_hex {
            themes.themes[sel_index].colors.light = trimmed.to_string();
            break;
        } else {
            println!("Invalid Hex!");
            continue;
        }
    }

    println!("Updated Light Color!");
}

fn edit_active(themes: &mut Themes, sel_index: usize) {
    println!("Name: {}", themes.themes[sel_index].name);
    println!(
        "Current Active Color: {}",
        themes.themes[sel_index].colors.secondary
    );
    println!("Enter Light Color Hex:");

    let mut input: String = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!");

        let trimmed = input.trim();

        let is_hex = trimmed.starts_with('#')
            && trimmed.len() == 7
            && trimmed[1..].chars().all(|c| c.is_ascii_hexdigit());

        if is_hex {
            themes.themes[sel_index].colors.active = trimmed.to_string();
            break;
        } else {
            println!("Invalid Hex!");
            continue;
        }
    }

    println!("Updated Active Color!");
}
