use std::io::Cursor;

use clap::Parser;
use image::{DynamicImage, ImageReader, codecs::jpeg::JpegEncoder};
use thiserror::Error;

#[derive(Parser, Debug)]
struct CommandLineArguments {
    input_path: String,
    output_path: String,
    #[arg(short, long, default_value_t = 1)]
    repetitions: u8,
    #[arg(short, long, default_value_t = 1)]
    quality: u8,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("{context}: {source}")]
    IoError {
        context: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{context}: {source}")]
    ImageError {
        context: String,
        #[source]
        source: image::ImageError,
    },
}

fn repeated_jpeg_encode(
    image: DynamicImage,
    repetitions: u8,
    quality: u8,
) -> Result<Vec<u8>, AppError> {
    let rgb_image = image.into_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut raw_rgb = rgb_image.into_raw();
    let mut jpeg_buffer = Vec::with_capacity(2 * 1024 * 1024); // 2 megabytes

    for i in 1..=repetitions {
        println!("Repetition {}...", i);

        jpeg_buffer.clear();
        let mut writer = Cursor::new(&mut jpeg_buffer);

        let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut writer, quality);
        jpeg_encoder
            .encode(&raw_rgb, width, height, image::ColorType::Rgb8.into())
            .map_err(|source| AppError::ImageError {
                context: format!("JPEG encoding failed at repetition {}", i),
                source,
            })?;

        raw_rgb = image::load_from_memory_with_format(&jpeg_buffer, image::ImageFormat::Jpeg)
            .map_err(|source| AppError::ImageError {
                context: format!("JPEG encoding failed while preparing for repetition {}", i),
                source,
            })?
            .into_rgb8()
            .into_raw();
    }

    Ok(jpeg_buffer)
}

fn load_image(path: String) -> Result<DynamicImage, AppError> {
    ImageReader::open(&path)
        .map_err(|source| AppError::IoError {
            context: format!("Failed to open image at path {}", path),
            source,
        })?
        .decode()
        .map_err(|source| AppError::ImageError {
            context: format!("Failed to decode image at path {}", path),
            source,
        })
}

fn main() -> Result<(), AppError> {
    let args = CommandLineArguments::parse();

    let original_image = load_image(args.input_path)?;
    let jpeg_data = repeated_jpeg_encode(original_image, args.repetitions, args.quality)?;

    std::fs::write(&args.output_path, jpeg_data).map_err(|source| AppError::IoError {
        context: format!("Failed to write output to {:?}", args.output_path),
        source,
    })?;

    Ok(())
}
