pub fn round_down(decimal_places: u32, value: f32) -> f32 {
    let precision_value = 10_f32.powf(decimal_places as f32);
    (value * precision_value).floor() / precision_value
}

pub fn round_up(decimal_places: u32, value: f32) -> f32 {
    let precision_value = 10_f32.powf(decimal_places as f32);
    (value * precision_value).ceil() / precision_value
}
