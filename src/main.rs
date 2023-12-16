mod image_scanner;

use image::GenericImageView;
use image_scanner::ImageScanner;

fn main() {
    let scanner = ImageScanner {};

    let img = scanner.load_image("assets/images/crewmate.png").unwrap();

    let (width, height) = img.dimensions();

    println!("Loaded image with width: {width}, height: {height}");
}
