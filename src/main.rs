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

fn compress_image_repeatedly(
    mut image: DynamicImage,
    repetitions: u8,
    quality: u8,
) -> Result<DynamicImage, AppError> {
    for i in 1..=repetitions {
        println!("Repetition {}...", i);

        let mut writer = Cursor::new(Vec::<u8>::new());
        let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut writer, quality);
        jpeg_encoder
            .encode_image(&image)
            .map_err(|source| AppError::ImageError {
                context: "Failed to encode image".to_string(),
                source,
            })?;

        writer.set_position(0);
        image = image::load(&mut writer, image::ImageFormat::Jpeg).map_err(|source| {
            AppError::ImageError {
                context: "Failed to reload compressed image".to_string(),
                source,
            }
        })?;
    }

    Ok(image)
}

fn main() -> Result<(), AppError> {
    let command_line_arguments = CommandLineArguments::parse();

    let image = ImageReader::open(&command_line_arguments.input_path)
        .map_err(|source| AppError::IoError {
            context: format!(
                "Failed to open image at path {}",
                command_line_arguments.input_path
            ),
            source,
        })?
        .decode()
        .map_err(|source| AppError::ImageError {
            context: format!(
                "Failed to decode image at path {}",
                command_line_arguments.input_path
            ),
            source,
        })?;

    let image = compress_image_repeatedly(
        image,
        command_line_arguments.repetitions,
        command_line_arguments.quality,
    )?;

    image
        .save(&command_line_arguments.output_path)
        .map_err(|source| AppError::ImageError {
            context: format!(
                "Failed to save image to path {}",
                command_line_arguments.output_path
            ),
            source,
        })?;

    Ok(())
}
