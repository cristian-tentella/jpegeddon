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
    mut image: DynamicImage,
    repetitions: u8,
    quality: u8,
) -> Result<DynamicImage, AppError> {
    for i in 1..=repetitions {
        println!("Repetition {}...", i);
        image = jpeg_encode(image, quality)?;
    }

    Ok(image)
}

fn jpeg_encode(image: DynamicImage, quality: u8) -> Result<DynamicImage, AppError> {
    let mut writer = Cursor::new(Vec::<u8>::new());
    let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut writer, quality);

    jpeg_encoder
        .encode_image(&image)
        .map_err(|source| AppError::ImageError {
            context: "Failed to encode image".to_string(),
            source,
        })?;

    writer.set_position(0);
    let encoded_image = image::load(&mut writer, image::ImageFormat::Jpeg).map_err(|source| {
        AppError::ImageError {
            context: "Failed to reload compressed image".to_string(),
            source,
        }
    })?;

    Ok(encoded_image)
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

fn save_image(image: DynamicImage, path: String) -> Result<(), AppError> {
    image.save(&path).map_err(|source| AppError::ImageError {
        context: format!("Failed to save image to path {}", path),
        source,
    })
}

fn main() -> Result<(), AppError> {
    let command_line_arguments = CommandLineArguments::parse();

    save_image(
        repeated_jpeg_encode(
            load_image(command_line_arguments.input_path)?,
            command_line_arguments.repetitions,
            command_line_arguments.quality,
        )?,
        command_line_arguments.output_path,
    )?;

    Ok(())
}
