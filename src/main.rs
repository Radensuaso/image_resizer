use std::env;
use std::fs;
use std::path::Path;
use image::imageops::FilterType;
use image::GenericImageView;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: image_resizer <keyword> <mb> <input_image> <output_image>");
        return;
    }

    let keyword = &args[1];
    let target_mb: f64 = args[2].parse().expect("Invalid number for MB");
    let input_image = &args[3];
    let output_image = &args[4];

    if keyword == "resize" {
        let input_path = Path::new(input_image);
        let output_path = Path::new(output_image);

        resize_image_to_mb(input_path, output_path, target_mb);
    } else {
        eprintln!("Unknown keyword: {}", keyword);
    }
}

fn resize_image_to_mb(input_path: &Path, output_path: &Path, target_mb: f64) {
    let img = image::open(input_path).expect("Failed to open input image");

    let initial_size = img.dimensions();
    let target_size = (target_mb * 1024.0 * 1024.0) as u64;

    let mut width = initial_size.0;
    let mut height = initial_size.1;

    // Iteratively resize until the file is smaller than target_size
    let mut resized_img = img.clone();
    loop {
        resized_img = resized_img.resize_exact(width, height, FilterType::Lanczos3);

        // Save the resized image temporarily to check the file size
        resized_img.save(output_path).expect("Failed to save resized image");

        let current_size = fs::metadata(output_path).expect("Failed to get file metadata").len();

        if current_size <= target_size {
            println!("Resized image saved to {}", output_path.display());
            break;
        }

        // Reduce dimensions by 10% for next iteration
        width = (width as f64 * 0.9) as u32;
        height = (height as f64 * 0.9) as u32;
    }
}
