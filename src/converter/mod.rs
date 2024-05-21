pub mod webp;

use crate::{converter::webp::encode_webp, format::ImageFormat, utils::is_supported};
use image::io::Reader;
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Processes and encodes images in a given directory to the specified image format.
pub fn convert_images(
    dir_path: &Path,
    output: &Option<String>,
    img_format: &ImageFormat,
) -> Result<(), String> {
    if dir_path.is_dir() {
        let entries: Vec<PathBuf> = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory: {}", e))?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();

        entries.par_iter().try_for_each(|path| {
            if path.is_dir() {
                convert_images(path, output, img_format)
            } else if is_supported(path, img_format) {
                convert_image(path, output, img_format)
            } else {
                Ok(())
            }
        })?;
    }
    Ok(())
}

/// Encodes an image to the specified image format and saves it to the specified output directory.
fn convert_image(
    input_path: &Path,
    output_dir: &Option<String>,
    img_format: &ImageFormat,
) -> Result<(), String> {
    let image_reader =
        Reader::open(input_path).map_err(|e| format!("Failed to open image: {}", e))?;
    let image = image_reader
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let image_data = match img_format {
        ImageFormat::Webp => encode_webp(&image)?,
        _ => return Err("Unsupported image format".to_string()),
    };

    let ext = img_format.extension();
    let output_path = if let Some(output_dir) = output_dir {
        Path::new(&output_dir)
            .join(input_path.file_stem().unwrap())
            .with_extension(ext)
    } else {
        input_path.with_extension(ext)
    };

    fs::write(output_path.clone(), image_data)
        .map_err(|e| format!("Failed to write image file: {}", e))?;

    println!("Generated: {}", output_path.display());

    Ok(())
}
