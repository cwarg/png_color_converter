use png::Reader;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const RGB_DIVISOR: f32 = 255.0; 

struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
struct Hsl {
    hue: f32,
    saturation: f32,
    lightness: f32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //let data = [0,0,0,0,0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    122,    200,    255,    255,    168,    233,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    122,    200,    255,    255,    51,    161,    240,    255,    122,    200,    255,    255,    168,    233,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    51,    161,    240,    255,    51,    161,    240,    255,    18,    68,    106,    255,    56,    116,    160,    255,    122,    200,    255,    255,    122,    200,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    122,    200,    255,    255,    51,    161,    240,    255,    18,    68,    106,    255,    51,    161,    240,    255,    168,    233,    255,    255,    56,    116,    160,    255,    122,    200,    255,    255,    168,    233,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    51,    161,    240,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    122,    200,    255,    255,    168,    233,    255,    255,    18,    68,    106,    255,    51,    161,    240,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    51,    161,    240,    255,    18,    68,    106,    255,    122,    200,    255,    255,    122,    200,    255,    255,    122,    200,    255,    255,    51,    161,    240,    255,    18,    68,    106,    255,    51,    161,    240,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    51,    161,    240,    255,    56,    116,    160,    255,    122,    200,    255,    255,    122,    200,    255,    255,    51,    161,    240,    255,    51,    161,    240,    255,    18,    68,    106,    255,    122,    200,    255,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    122,    200,    255,    255,    56,    116,    160,    255,    122,    200,    255,    255,    51,    161,    240,    255,    51,    161,    240,    255,    51,    161,    240,    255,    56,    116,    160,    255,    51,    161,    240,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    28,    59,    82,    255,    122,    200,    255,    255,    56,    116,    160,    255,    51,    161,    240,    255,    51,    161,    240,    255,    51,    161,    240,    255,    122,    200,    255,    255,    18,    68,    106,    255,    51,    161,    240,    255,    1,    19,    33,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    28,    59,    82,    255,    17,    88,    219,    255,    17,    88,    219,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    51,    161,    240,    255,    17,    88,    219,    255,    56,    116,    160,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    51,    161,    240,    255,    17,    88,    219,    255,    51,    161,    240,    255,    51,    161,    240,    255,    1,    19,    33,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    17,    88,    219,    255,    51,    161,    240,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    1,    19,    33,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,];
    //let dst: Vec<&[i32]> = data.chunks(8).collect();
    //file_reader(&args[1]);
    // file_writer(&args[2]);
    let mut rgba = Rgba {
        r: 133,
        g: 205,
        b: 33,
        a: 255,
    };
    dbg!(calculate_hue_from_rgb(&rgba));
}

fn file_reader(filepath: &String) {
    let decoder = png::Decoder::new(File::open(filepath).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    let in_animation = reader.info().frame_control().is_some();
    dbg!(bytes);
}

fn file_writer(filepath: &String) {
    let path = Path::new(filepath);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 16, 16);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    //let mut data: [u8; 1024] = [0,0,0,0,0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    122,    200,    255,    255,    168,    233,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    122,    200,    255,    255,    51,    161,    240,    255,    122,    200,    255,    255,    168,    233,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    51,    161,    240,    255,    51,    161,    240,    255,    18,    68,    106,    255,    56,    116,    160,    255,    122,    200,    255,    255,    122,    200,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    122,    200,    255,    255,    51,    161,    240,    255,    18,    68,    106,    255,    51,    161,    240,    255,    168,    233,    255,    255,    56,    116,    160,    255,    122,    200,    255,    255,    168,    233,    255,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    51,    161,    240,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    122,    200,    255,    255,    168,    233,    255,    255,    18,    68,    106,    255,    51,    161,    240,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    51,    161,    240,    255,    18,    68,    106,    255,    122,    200,    255,    255,    122,    200,    255,    255,    122,    200,    255,    255,    51,    161,    240,    255,    18,    68,    106,    255,    51,    161,    240,    255,    28,    59,    82,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    6,    33,    55,    255,    51,    161,    240,    255,    56,    116,    160,    255,    122,    200,    255,    255,    122,    200,    255,    255,    51,    161,    240,    255,    51,    161,    240,    255,    18,    68,    106,    255,    122,    200,    255,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    122,    200,    255,    255,    56,    116,    160,    255,    122,    200,    255,    255,    51,    161,    240,    255,    51,    161,    240,    255,    51,    161,    240,    255,    56,    116,    160,    255,    51,    161,    240,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    28,    59,    82,    255,    122,    200,    255,    255,    56,    116,    160,    255,    51,    161,    240,    255,    51,    161,    240,    255,    51,    161,    240,    255,    122,    200,    255,    255,    18,    68,    106,    255,    51,    161,    240,    255,    1,    19,    33,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    28,    59,    82,    255,    17,    88,    219,    255,    17,    88,    219,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    51,    161,    240,    255,    17,    88,    219,    255,    56,    116,    160,    255,    18,    68,    106,    255,    51,    161,    240,    255,    122,    200,    255,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    51,    161,    240,    255,    17,    88,    219,    255,    51,    161,    240,    255,    51,    161,    240,    255,    1,    19,    33,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    17,    88,    219,    255,    51,    161,    240,    255,    6,    33,    55,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    1,    19,    33,    255,    1,    19,    33,    255,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,    0,];
    let mut data: [u8; 1024] = [0; 1024];
    let init = [255, 255, 255, 255, 255, 255, 255, 255];
    data[..init.len()].copy_from_slice(&init);
    //    for i in 0..data.len() {
    //       if data[i] != 0 {
    //            data[i] = data[i].wrapping_add(5);
    //        }
    //    }
    writer.write_image_data(&data).unwrap();
}

fn calculate_hue_from_rgb(rgba: &Rgba) -> Hsl {
    let converted_r = rgba.r as f32 / RGB_DIVISOR;
    let converted_g = rgba.g as f32 / RGB_DIVISOR;
    let converted_b = rgba.b as f32 / RGB_DIVISOR;

    let mut max = f32::max(converted_r, converted_g);
    max = f32::max(max, converted_b);

    let mut min = f32::min(converted_r, converted_g);
    min = f32::min(min, converted_b);

    let difference:f32 = max - min;
    let mut hue: f32;
   
    if max == converted_r {
        println!("R is Max");
        let segment = (converted_g - converted_b) / difference;
        let shift = 0.0 / 60.0;
        hue = (segment + shift) * 60.0;
    } else if max == converted_g {
        println!("G is Max");
        let segment = (converted_b - converted_r) / difference;
        let shift = 120.0 / 60.0;
        hue = (segment + shift) * 60.0;
    } else if max == converted_b {
        println!("B is max");
        let segment = (converted_r - converted_g) / difference;
        let shift = 240.0 / 60.0;
        hue = (segment + shift) * 60.0;
    } else {
        hue = 0.0;
    }

    if hue < 0.0 {
        hue += 360.0;
    }

    let lightness = (max + min) / 2.0;


    println!("HUE waduwad");
    Hsl{
        hue: hue,
        saturation: 0.0,
        lightness: 0.0,
    }
}
