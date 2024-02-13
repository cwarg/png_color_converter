const RGB_DIVISOR: f32 = 255.0;
const HSL_DIVISOR: f32 = 100.0;

#[derive(Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug)]
pub struct Hsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

pub fn calculate_hsl_from_rgb(rgba: Rgba) -> Hsl {
    let converted_r = rgba.r as f32 / RGB_DIVISOR;
    let converted_g = rgba.g as f32 / RGB_DIVISOR;
    let converted_b = rgba.b as f32 / RGB_DIVISOR;

    let mut max = f32::max(converted_r, converted_g);
    max = f32::max(max, converted_b);

    let mut min = f32::min(converted_r, converted_g);
    min = f32::min(min, converted_b);

    let difference: f32 = max - min;
    let mut hue: f32;

    if max == converted_r {
        let segment = (converted_g - converted_b) / difference;
        let shift = 0.0 / 60.0;
        hue = (segment + shift) * 60.0;
    } else if max == converted_g {
        let segment = (converted_b - converted_r) / difference;
        let shift = 120.0 / 60.0;
        hue = (segment + shift) * 60.0;
    } else if max == converted_b {
        let segment = (converted_r - converted_g) / difference;
        let shift = 240.0 / 60.0;
        hue = (segment + shift) * 60.0;
    } else {
        hue = 0.0;
    }

    if hue < 0.0 {
        hue += 360.0;
    }

    let mut lightness = (max + min) / 2.0;
    let mut saturation = difference / ((1.0 - (2.0 * lightness - 1.0).abs()));

    saturation *= 100.0;
    lightness *= 100.0;

    Hsl {
        hue,
        saturation,
        lightness,
    }
}

pub fn calculate_rgb_from_hsl(hsl: Hsl) -> Rgba {
    let converted_h = hsl.hue / 60.0;
    let converted_s = hsl.saturation / HSL_DIVISOR;
    let converted_l = hsl.lightness / HSL_DIVISOR;

    let mut chroma = (2.0 * converted_l - 1.0).abs();
    chroma = (1.0 - chroma) * converted_s;

    let x = chroma * (1.0 - ((converted_h) % 2.0 - 1.0).abs());
    let lightness_match = converted_l - (chroma / 2.0);

    let mut r: f32 = 0.0;
    let mut g: f32 = 0.0;
    let mut b: f32 = 0.0;

    if 0.0 <= converted_h && converted_h <= 1.0 {
        r = chroma; g = x; b = 0.0;
    } else if 1.0 <= converted_h && converted_h <= 2.0 {
        r = x; g = chroma; b = 0.0;
    } else if 2.0 <= converted_h && converted_h <= 3.0 {
        r = 0.0; g = chroma; b = x;
    } else if 3.0 <= converted_h && converted_h <= 4.0 {
        r = 0.0; g = x; b = chroma;
    } else if 4.0 <= converted_h && converted_h <= 5.0 {
        r = x; g = 0.0; b = chroma;
    } else if 5.0 <= converted_h && converted_h <= 6.0 {
        r = chroma; g = 0.0; b = x;
    }

    let final_r: u8 = ((r + lightness_match) * 255.0).round() as u8;
    let final_g: u8 = ((g + lightness_match) * 255.0).round() as u8;
    let final_b: u8 = ((b + lightness_match) * 255.0).round() as u8;
    let final_a: u8;

    if final_r == 0 && final_g == 0 && final_b == 0 {
        final_a = 0;
    } else {
        final_a = 255;
    }

    Rgba {
        r: final_r,
        g: final_g,
        b: final_b,
        a: final_a,
    }
}