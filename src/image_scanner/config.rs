use image::Rgba;

pub struct Config {
    pub searched_color: Rgba<u8>,
    pub pattern_extracting_tolerance: u8,
    pub pattern_searching_tolerance: u8,
}

impl Config {
    pub fn new(
        searched_color: Rgba<u8>,
        pattern_extracting_tolerance: u8,
        pattern_searching_tolerance: u8,
    ) -> Self {
        Config {
            searched_color,
            pattern_extracting_tolerance,
            pattern_searching_tolerance,
        }
    }

    pub fn new_default() -> Self {
        Config {
            searched_color: Rgba([0, 0, 0, 0]),
            pattern_extracting_tolerance: 1,
            pattern_searching_tolerance: 0,
        }
    }
}
