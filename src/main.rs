use image::GenericImageView;
use kmeans_colors::get_kmeans_hamerly;
use palette::{FromColor, Lab, Lch, Srgb};
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

    println!("Insert wallpaper path:");
    std::io::stdin().read_line(&mut path).expect("Dumb ass!");

    println!("Enter Theme Name:");
    std::io::stdin()
        .read_line(&mut theme_name)
        .expect("Dumb ass!");

    let file_name = theme_name.to_lowercase().replace(" ", "_");

    let img = image::open(path.trim()).unwrap();
    let resized_img = img.resize(200, 200, image::imageops::FilterType::Triangle);

    let pixels: Vec<Lab> = resized_img
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

    let result = get_kmeans_hamerly(16, 20, 0.005, false, &pixels, 42);

    let mut lch_colors: Vec<Lch> = result
        .centroids
        .iter()
        .map(|&lab| Lch::from_color(lab))
        .collect();

    lch_colors.sort_by(|a, b| b.chroma.partial_cmp(&a.chroma).unwrap());

    let mut active_lch = lch_colors[0];
    active_lch.l = active_lch.l.clamp(60.0, 80.0);

    let mut primary_lch = lch_colors[1];
    primary_lch.l = 20.0;
    primary_lch.chroma = primary_lch.chroma.clamp(55.0, 65.0);

    let mut secondary_lch = lch_colors[2];
    secondary_lch.l = 70.0;
    secondary_lch.chroma = secondary_lch.chroma.clamp(30.0, 50.0);

    let mut light_lch = primary_lch;
    light_lch.l = 70.0;
    light_lch.chroma = 20.0;

    let to_hex = |lab: Lab| -> String {
        let rgb = Srgb::from_color(lab);
        format!(
            "#{:02x}{:02x}{:02x}",
            (rgb.red * 255.0) as u32,
            (rgb.green * 255.0) as u32,
            (rgb.blue * 255.0) as u32
        )
    };

    let active = to_hex(Lab::from_color(active_lch));
    let primary = to_hex(Lab::from_color(primary_lch));
    let secondary = to_hex(Lab::from_color(secondary_lch));
    let light = to_hex(Lab::from_color(light_lch));

    println!("Successful color extraction!");

    println!("Light: {:#?}", light);
    println!("Active: {:#?}", active);
    println!("Primary: {:#?}", primary);
    println!("Secondary: {:#?}", secondary);

    let upscalled_image = img.resize(1920, 1080, image::imageops::Nearest);

    let image_ext = Path::new(&path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    let image_file_name = format!("{}.{}", file_name.trim(), image_ext.trim());

    let dest_folder = format!("{}/.config/colorSchemes/wallpapers/", &home);
    let dest_path = Path::new(&dest_folder.trim()).join(&image_file_name.trim());

    upscalled_image.save(&dest_path).unwrap();

    let colors = ThemeColors {
        primary: primary,
        secondary: secondary,
        light: light,
        active: active,
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

    println!("File Uploaded Successful!");
    println!("Done!")
}
