use image::{io::Reader as ImageReader, DynamicImage};
use std::{error::Error, path::Path};

pub struct ImageIO {}

impl ImageIO {
    pub fn load_image(path: &str) -> Result<DynamicImage, Box<dyn Error>> {
        let search_img = ImageReader::open(Path::new(path))?.decode()?;
        Ok(search_img)
    }

    pub fn load_multiple_images(paths: Vec<&str>) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
        let mut images = Vec::new();

        for path in paths {
            if ImageIO::is_path_valid(path) {
                if let Ok(image) = ImageIO::load_image(path) {
                    images.push(image);
                }
            }
        }

        Ok(images)
    }

    pub fn is_path_valid(path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn are_paths_valid(paths: Vec<&str>) -> bool {
        paths.iter().all(|&path| ImageIO::is_path_valid(path))
    }
}
