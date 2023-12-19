mod image_scanner;

use image::io::Reader as ImageReader;
use image_scanner::ImageScanner;

use crate::image_scanner::Config;

fn main() {
    let config = Config::new_default();

    let scanner = ImageScanner::new(config);

    let search_image_path = "assets/images/crewmate.png";
    let search_img = ImageReader::open(search_image_path)
        .unwrap()
        .decode()
        .unwrap();
    let img_path = "assets/images/crewmate_with_borders.png";
    let img = ImageReader::open(img_path).unwrap().decode().unwrap();

    let search_pattern = scanner.create_pattern_from_image(search_img);

    let found_patterns = scanner.scan_image_for_patterns(&search_pattern, &img);

    println!("{:?}", found_patterns);
}
