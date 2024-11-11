use image::imageops::FilterType;
use image::GenericImageView;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage:");
        eprintln!("  image_resizer resize <mb> <input_image> <output_image>");
        eprintln!("  image_resizer resize_bulk <mb> <input_folder> <output_folder>");
        return;
    }

    let keyword = &args[1];
    let target_mb: f64 = args[2].parse().expect("Invalid number for MB");

    if keyword == "resize" {
        let input_image = &args[3];
        let output_image = &args[4];
        resize_image_to_mb(Path::new(input_image), Path::new(output_image), target_mb);
    } else if keyword == "resize_bulk" {
        let input_folder = &args[3];
        let output_folder = &args[4];
        resize_images_in_folder(input_folder, output_folder, target_mb);
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

        // Reduce dimensions by 10% for next iteration
        width = (width as f64 * 0.9) as u32;
        height = (height as f64 * 0.9) as u32;
    }
}

fn resize_images_in_folder(input_folder: &str, output_folder: &str, target_mb: f64) {
    let input_path = Path::new(input_folder);
    let output_path = Path::new(output_folder);

    // Create output folder if it doesn't exist
    if !output_path.exists() {
        fs::create_dir_all(output_path).expect("Failed to create output folder");
    }

    // Iterate over all files in the input folder
    for entry in fs::read_dir(input_path).expect("Failed to read input folder") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();

        if path.is_file() {
            // Generate the output file path in the output folder
            let output_file_path =
                output_path.join(path.file_name().expect("Failed to get file name"));

            // Resize the image and save it to the output folder
            resize_image_to_mb(&path, &output_file_path, target_mb);
        }
    }

    println!(
        "Bulk resize completed. All images saved to {}",
        output_folder
    );
}
