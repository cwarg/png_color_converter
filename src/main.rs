mod color_processing;
mod file_processing;

use clap::Parser;
use crate::file_processing::process_image;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "Path to input png to convert.")]
    input_file: String,

    #[arg(short, long, value_name = "Output path (including file name) where you'd like the converted file to be saved.")]
    output_file: String,

    #[arg(short, long, value_name = "Choose one of the following: red, orange, yellow, green, blue, indigo, violet...")]
    color: String,

    #[arg(short, long, value_name = "Optionally pick a specific hue value (f32 between 0.0 and 360.0) rather than using one of the preset ones.")]
    value: Option<f32>,
}

fn main() {
    let args = Args::parse();
    process_image(&args.input_file, &args.output_file, &args.color, &args.value);
}
