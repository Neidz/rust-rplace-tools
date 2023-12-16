use std::path::Path;

use image::DynamicImage;

use super::pattern::Pattern;

pub struct ImageScanner {}

impl ImageScanner {
    pub fn load_image(&self, path: &str) -> Result<DynamicImage, image::ImageError> {
        let path = Path::new(path);

        image::open(path)
    }

    pub fn load_images(&self, paths: Vec<&str>) -> Result<Vec<DynamicImage>, image::ImageError> {
        let mut images = Vec::new();

        for path_str in paths {
            let path = Path::new(path_str);
            let image = image::open(path)?;
            images.push(image);
        }

        Ok(images)
    }

    pub fn image_to_patterns(&self, image: DynamicImage) -> Vec<Pattern> {
        todo!()
    }

    pub fn images_to_patterns(&self, images: Vec<DynamicImage>) -> Vec<Vec<Pattern>> {
        todo!()
    }
}
