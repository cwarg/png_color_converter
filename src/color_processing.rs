use crate::utilities::round_down;

const RGB_DIVISOR: f32 = 255.0;

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

pub fn calculate_hsl_from_rgb(rgba: &Rgba) -> Hsl {
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
        hue: round_down(2, hue),
        saturation: round_down(2, saturation),
        lightness: round_down(2, lightness),
    }
}

pub fn calculate_rgb_from_hsl(hsl: &Hsl) -> Rgba {
    Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }
}