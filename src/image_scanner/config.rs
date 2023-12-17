use image::Rgba;

pub struct Config {
    pub searched_color: Rgba<u8>,
    pub tolerance: u8,
}

impl Config {
    pub fn new(searched_color: Rgba<u8>, tolerance: u8) -> Self {
        Config {
            searched_color,
            tolerance,
        }
    }
}
