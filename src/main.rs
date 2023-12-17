mod image_scanner;

use image::io::Reader as ImageReader;
use image::Rgba;
use image_scanner::ImageScanner;

use crate::image_scanner::Config;

fn main() {
    let config = Config::new(Rgba([0, 0, 0, 0]), 1);

    let scanner = ImageScanner::new(config);

    let image_path = "assets/images/crewmate.png";
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let pattern = scanner.create_pattern_from_image(img);

    println!("{:?}", pattern);
}
