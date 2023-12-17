use std::path::Path;

use image::DynamicImage;

use super::{pattern::Pattern, Config};

pub struct ImageScanner {
    config: Config,
}

impl ImageScanner {
    pub fn new(config: Config) -> Self {
        ImageScanner { config }
    }

    pub fn create_pattern_from_image(&self, image: DynamicImage) -> Pattern {
        Pattern::from_image(image, &self.config.searched_color, &self.config.tolerance)
    }

    pub fn scan_image_for_patterns(&self, image: DynamicImage) -> Vec<Pattern> {
        todo!()
    }

    pub fn scan_images_to_patterns(&self, images: Vec<DynamicImage>) -> Vec<Vec<Pattern>> {
        todo!()
    }
}
