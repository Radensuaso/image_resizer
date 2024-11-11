use image::imageops::FilterType;
use image::GenericImageView;
use std::fs;
use std::path::Path;

pub fn resize_image_to_mb(input_path: &Path, output_path: &Path, target_mb: f64) {
    let img = image::open(input_path).expect("Failed to open input image");

    let initial_size = img.dimensions();
    let target_size = (target_mb * 1024.0 * 1024.0) as u64;

    let mut width = initial_size.0;
    let mut height = initial_size.1;

    let mut resized_img = img.clone();
    loop {
        resized_img = resized_img.resize_exact(width, height, FilterType::Lanczos3);
        resized_img
            .save(output_path)
            .expect("Failed to save resized image");

        let current_size = fs::metadata(output_path)
            .expect("Failed to get file metadata")
            .len();

        if current_size <= target_size {
            println!("Resized image saved to {}", output_path.display());
            break;
        }

        width = (width as f64 * 0.9) as u32;
        height = (height as f64 * 0.9) as u32;
    }
}
