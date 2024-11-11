mod resize_image;
mod resize_bulk_images;
mod web_crawler;
mod weather;

use resize_image::resize_image_to_mb;
use resize_bulk_images::resize_images_in_folder;
use std::env;
use std::path::Path;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  tool_kit resize_image <mb> <input_image> <output_image>");
        eprintln!("  tool_kit resize_bulk_images <mb> <input_folder> <output_folder>");
        eprintln!("  tool_kit web_crawler <URL> <max_depth>");
        eprintln!("  tool_kit weather <city>");
        return;
    }

    let keyword = &args[1];

    if keyword == "resize_image" && args.len() == 5 {
        let target_mb: f64 = args[2].parse().expect("Invalid number for MB");
        let input_image = &args[3];
        let output_image = &args[4];
        resize_image_to_mb(Path::new(input_image), Path::new(output_image), target_mb);
    } else if keyword == "resize_bulk_images" && args.len() == 5 {
        let target_mb: f64 = args[2].parse().expect("Invalid number for MB");
        let input_folder = &args[3];
        let output_folder = &args[4];
        resize_images_in_folder(input_folder, output_folder, target_mb);
    } else if keyword == "web_crawler" && args.len() == 4 {
        let start_url = &args[2];
        let max_depth: usize = args[3].parse().expect("Invalid number for max depth");
        if let Err(e) = web_crawler::crawl(start_url, max_depth).await {
            eprintln!("Error during web crawling: {}", e);
        }
    } else if keyword == "weather" && args.len() == 3 {
        let city = &args[2];
        if let Err(e) = weather::fetch_weather(city).await {
            eprintln!("Failed to fetch weather data: {}", e);
        }
    } else {
        eprintln!("Unknown or invalid command");
    }
}
