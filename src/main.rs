use serde_json::Value as JsonValue;
use wallpaper;
use std::env;
use std::fs;
use chrono::prelude::*;
// use std::path::Path;

fn main() {
    loop {
        let contents = fs::read_to_string("src/json/last_date.json")
            .expect("Something went wrong reading the file");

        let parsed = serde_json::from_str(&contents);

        if parsed.is_ok() {
            let p: JsonValue = parsed.unwrap();
            println!("The name is {}", p["date"].as_str().unwrap());
            println!("{:?}", wallpaper::get());
            let date = p["date"].as_str().unwrap();
            if check_date(Local::now(), date) {
                set_background(date).expect("Can not set background!");
            }
        } else {
            println!("Sorry! Could not parse JSON :(");
        }
    }
}

fn set_background(date: &str) -> std::io::Result<()> {
    let path = env::current_dir()?;
    let formatted = format!("{}\\src\\images\\{}{}", path.display(), date, ".jpg");
    wallpaper::set_from_path(&formatted).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    Ok(())
}

fn check_date(current_date: DateTime<Local>, date: &str) -> bool {
    let current_date_formatted = current_date.format("%Y-%m-%d");
    println!("{}", current_date_formatted);
    if current_date_formatted.to_string() != date.to_string() {
        return true
    } else {
        return false
    }
}
