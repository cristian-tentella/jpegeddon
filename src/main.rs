use clap::Parser;

#[derive(clap::Parser, Debug)]
struct CommandLineArguments {
    input_path: String,
    output_path: String,
    #[arg(short, long, default_value_t = 1)]
    repetitions: u8,
    #[arg(short, long, default_value_t = 75)]
    quality: u8,
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();
}
