use chrono::prelude::*;
use serde_json::Value as JsonValue;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;

#[tokio::main]
async fn main() {
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
                download_image(date).await;
                set_background(date).expect("Could not set background!");
            }
        } else {
            println!("Sorry! Could not parse JSON :(");
        }
    }
}

async fn download_image(date: &str) {
    let token = "unsplash-api-access-key";
    match reqwest::get(format!(
        "https://api.unsplash.com/photos/random?count=1&client_id={}",
        token
    ))
    .await
    {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                match response.text().await {
                    Ok(text) => {
                        let parsed_text: JsonValue = serde_json::from_str(&text).unwrap();
                        // let image_description = parsed_text[0]["description"].as_str().unwrap();
                        let image_link = parsed_text[0]["urls"]["raw"].as_str().unwrap();
                        download(image_link, date).await;
                    }
                    Err(_) => println!("Could not load image data"),
                }
            } else {
                println!("Could not load web data : {}", response.status());
            }
        }
        Err(_) => println!("Error downloading"),
    }
}

async fn download(image_link: &str, image_name: &str) -> Result<(), Box<dyn Error>> {
    let rsp = reqwest::get(image_link).await?;
    let content = rsp.bytes().await?;

    let mut dest = fs::File::create(format!("src/images/{}{}", image_name, ".jpg"))?;

    let mut pos = 0;
    while pos < content.len() {
        let bytes_written = dest.write(&content[pos..])?;
        pos += bytes_written;
    }
    Ok(())
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
    current_date_formatted.to_string() != *date.to_string()
}
