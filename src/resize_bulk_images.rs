use std::fs;
use std::path::Path;

use crate::resize_image::resize_image_to_mb;

pub fn resize_images_in_folder(input_folder: &str, output_folder: &str, target_mb: f64) {
    let input_path = Path::new(input_folder);
    let output_path = Path::new(output_folder);

    if !output_path.exists() {
        fs::create_dir_all(output_path).expect("Failed to create output folder");
    }

    for entry in fs::read_dir(input_path).expect("Failed to read input folder") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();

        if path.is_file() {
            let output_file_path =
                output_path.join(path.file_name().expect("Failed to get file name"));
            resize_image_to_mb(&path, &output_file_path, target_mb);
        }
    }

    println!(
        "Bulk resize completed. All images saved to {}",
        output_folder
    );
}
