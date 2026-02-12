use image::GenericImageView;
use kmeans_colors::get_kmeans_hamerly;
use palette::{FromColor, Lab, Srgb};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct ThemeColors {
    primary: String,
    secondary: String,
    light: String,
    active: String,
}

#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    selected: bool,
    quickshell_theme: String,
    colors: ThemeColors,
    wallpaper: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    themes: Vec<Theme>,
}

fn main() {
    let home = std::env::var("HOME").unwrap();

    let config_path = format!("{}/.config/colorSchemes/themes.json", home);

    let json_file = fs::read_to_string(&config_path).unwrap();
    let mut config: Config = serde_json::from_str(&json_file).unwrap();

    let mut path = String::new();
    let mut theme_name = String::new();
    let mut file_name = String::new();

    println!("Insert wallpaper path:");
    std::io::stdin().read_line(&mut path).expect("Dumb ass!");

    println!("Enter Theme Name:");
    std::io::stdin()
        .read_line(&mut theme_name)
        .expect("Dumb ass!");

    println!("Enter Image File Name: (No Extension)");
    std::io::stdin()
        .read_line(&mut file_name)
        .expect("Dumb ass!");

    let img = image::open(path.trim()).unwrap();

    let pixels: Vec<Lab> = img
        .pixels()
        .map(|(_, _, p)| {
            let srgb = Srgb::new(
                p[0] as f32 / 255.0,
                p[1] as f32 / 255.0,
                p[2] as f32 / 255.0,
            );

            Lab::from_color(srgb)
        })
        .collect();

    let result = get_kmeans_hamerly(4, 20, 0.005, false, &pixels, 42);

    let lab_color_1: Lab = result.centroids[0];
    let lab_color_2: Lab = result.centroids[1];
    let lab_color_3: Lab = result.centroids[2];
    let lab_color_4: Lab = result.centroids[3];

    let color_1 = Srgb::from_color(lab_color_1);
    let color_2 = Srgb::from_color(lab_color_2);
    let color_3 = Srgb::from_color(lab_color_3);
    let color_4 = Srgb::from_color(lab_color_4);

    let hex_1 = format!(
        "#{:02x}{:02x}{:02x}",
        (color_1.red * 255.0) as u32,
        (color_1.green * 255.0) as u32,
        (color_1.blue * 255.0) as u32
    );
    let hex_2 = format!(
        "#{:02x}{:02x}{:02x}",
        (color_2.red * 255.0) as u32,
        (color_2.green * 255.0) as u32,
        (color_2.blue * 255.0) as u32
    );
    let hex_3 = format!(
        "#{:02x}{:02x}{:02x}",
        (color_3.red * 255.0) as u32,
        (color_3.green * 255.0) as u32,
        (color_3.blue * 255.0) as u32
    );
    let hex_4 = format!(
        "#{:02x}{:02x}{:02x}",
        (color_4.red * 255.0) as u32,
        (color_4.green * 255.0) as u32,
        (color_4.blue * 255.0) as u32
    );

    println!("1: {:#?}", hex_1);
    println!("2: {:#?}", hex_2);
    println!("3: {:#?}", hex_3);
    println!("4: {:#?}", hex_4);

    println!("Successful color extraction!");

    let image_ext = Path::new(&path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    let image_file_name = format!("{}.{}", file_name.trim(), image_ext.trim());

    let dest_folder = format!("{}/.config/colorSchemes/wallpapers/", &home);
    let dest_path = Path::new(&dest_folder.trim()).join(&image_file_name.trim());

    fs::copy(path.trim(), &dest_path).unwrap();

    let colors = ThemeColors {
        primary: hex_1,
        secondary: hex_2,
        light: hex_3,
        active: hex_4,
    };

    let theme = Theme {
        name: theme_name.trim().to_string(),
        quickshell_theme: "Default".to_string(),
        selected: false,
        colors: colors,
        wallpaper: image_file_name,
    };

    config.themes.push(theme);

    let updated_config = serde_json::to_string_pretty(&config).unwrap();

    fs::write(&config_path, &updated_config).unwrap();
}
