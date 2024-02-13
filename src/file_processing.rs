use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use crate::color_processing::{calculate_hsl_from_rgb, calculate_rgb_from_hsl, Hsl, Rgba};

const RED:f32 = 360.0;
const ORANGE:f32 = 39.0;
const YELLOW:f32 = 55.0;
const GREEN:f32 = 147.0;
const BLUE:f32 = 240.0;
const INDIGO:f32 = 300.0;
const VIOLET:f32 = 248.0;

pub fn process_image(input_filepath: &String, output_filepath: &String, color: &str, hue_value: &Option<f32>) {
    let decoder = png::Decoder::new(File::open(input_filepath).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes: &[u8] = &buf[..info.buffer_size()];

    let color = match color.to_lowercase().trim() {
        "red" => RED,
        "orange" => ORANGE,
        "yellow" => YELLOW,
        "green" => GREEN,
        "blue" => BLUE,
        "indigo" => INDIGO,
        "violet" => VIOLET,
        _ => panic!("Unhandled Color Has Been Entered")
    };
    let bytes = process_pixels(bytes, color);
    file_writer(output_filepath, bytes);
}

pub fn file_writer(filepath: &String, bytes: Vec<u8>) {
    let path = Path::new(filepath);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, 16, 16);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&bytes).unwrap();
}

pub fn process_pixels(bytes: &[u8], color: f32) -> Vec<u8> {
    let dst: Vec<&[u8]> = bytes.chunks(4).collect();
    let mut rgba_vec: Vec<Rgba> = vec![];
    for pixel in dst {
        rgba_vec.push(Rgba{r: pixel[0], g: pixel[1], b: pixel[2], a: pixel[3]});
    }
    let mut hsl_vec: Vec<Hsl> = vec![];
    for pixel in rgba_vec {
        if !(pixel.r == 0 && pixel.g == 0 && pixel.g == 0) {
            let current_pixel_as_hsl = calculate_hsl_from_rgb(pixel);
            hsl_vec.push(Hsl {
                hue: current_pixel_as_hsl.hue,
                saturation:current_pixel_as_hsl.saturation,
                lightness: current_pixel_as_hsl.lightness,
            })
        } else {
            hsl_vec.push(Hsl {hue: 0.0, saturation: 0.0, lightness: 0.0})
        }
    }
    rgba_vec = Vec::new();
    for hsl_pixel in &mut hsl_vec {
        if !(hsl_pixel.hue == 0.0) {
            hsl_pixel.hue = color;
        }
        let final_rgba_pixel = calculate_rgb_from_hsl(
            Hsl {
                hue: hsl_pixel.hue,
                saturation: hsl_pixel.saturation,
                lightness: hsl_pixel.lightness,
            });
        rgba_vec.push(final_rgba_pixel);
    }
    let mut final_array: Vec<u8> = vec![];
    for pixel in rgba_vec {
        final_array.push(pixel.r);
        final_array.push(pixel.g);
        final_array.push(pixel.b);
        final_array.push(pixel.a);
    }
    final_array
}