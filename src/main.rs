use std::io::Cursor;

use clap::Parser;
use image::{codecs::jpeg::JpegEncoder, ImageReader};

#[derive(Parser, Debug)]
struct CommandLineArguments {
    input_path: String,
    output_path: String,
    #[arg(short, long, default_value_t = 1)]
    repetitions: u8,
    #[arg(short, long, default_value_t = 1)]
    quality: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command_line_arguments = CommandLineArguments::parse();
    
    let mut image = ImageReader::open(&command_line_arguments.input_path)?.decode()?;    
    
    for i in 1..=command_line_arguments.repetitions {
        println!("Repetition {}...", i);
        
        let mut writer = Cursor::new(Vec::<u8>::new());
        let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut writer, command_line_arguments.quality);
        jpeg_encoder.encode_image(&image)?;

        writer.set_position(0);
        image = image::load(&mut writer, image::ImageFormat::Jpeg)?;
    }
    
    image.save(&command_line_arguments.output_path)?;

    Ok(())
}
