use image::DynamicImage;

use super::{pattern::Pattern, scan_image::scan_image, Config};

pub struct ImageScanner {
    config: Config,
}

impl ImageScanner {
    pub fn new(config: Config) -> Self {
        ImageScanner { config }
    }

    pub fn create_pattern(&self, image: DynamicImage) -> Pattern {
        Pattern::from_image(
            image,
            self.config.searched_color,
            self.config.pattern_extracting_tolerance,
        )
    }

    pub fn scan_image_for_patterns(
        &self,
        search_pattern: &Pattern,
        image: &DynamicImage,
    ) -> Vec<Pattern> {
        scan_image(
            image,
            search_pattern,
            self.config.pattern_searching_tolerance,
        )
    }

    pub fn scan_multiple_images_for_patterns(
        &self,
        images: &Vec<DynamicImage>,
    ) -> Vec<Vec<Pattern>> {
        todo!()
    }
}
