use std::fs;
use std::path::Path;

use crate::list::FunctionType;
use crate::list::list;
use crate::select::select;

pub fn delete(index: Option<usize>) {
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

    println!("Name: {}", themes.themes[sel_index].name);
    println!("Are you sure you wanna delete the theme?(Y/N(default)");

    loop {
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!");

        if input.trim() == "y" {
            break;
        } else {
            println!("Stopping!");
            return;
        }
    }

    if themes.themes[sel_index].selected {
        if sel_index == 0 {
            select(Some(1));
        } else {
            select(Some(0));
        }
    }

    let wallpaper_path: String = format!(
        "{}/.config/colorSchemes/wallpapers/{}",
        home, themes.themes[sel_index].wallpaper
    );

    if Path::new(&wallpaper_path).exists() {
        fs::remove_file(&wallpaper_path).expect("Couldnt delete wallpaper!");
    } else {
        println!("File already gone or never existed.");
    }

    themes.themes.remove(sel_index);

    let config_json =
        serde_json::to_string_pretty(&themes).expect("Couldnt convert config file to JSON!");

    fs::write(config_path, config_json).expect("Couldnt update config file!");

    println!("Deleted the theme!")
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
