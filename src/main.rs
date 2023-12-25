mod image_io;
mod image_scanner;

use std::time::Instant;

use image_io::ImageIO;
use image_scanner::ImageScanner;

use crate::image_scanner::Config;

fn main() {
    let start_time = Instant::now();
    let config = Config::new_default();

    let scanner = ImageScanner::new(config);

    let target_image = ImageIO::load_image("assets/images/crewmate.png").unwrap();
    let source_image = ImageIO::load_image("assets/images/final_2023_place.png").unwrap();

    let search_pattern = scanner.create_pattern(target_image);
    let found_patterns = scanner.scan_image_for_patterns(&search_pattern, &source_image);

    println!("{:?}", found_patterns.len());

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:.2?}", elapsed_time);
}
