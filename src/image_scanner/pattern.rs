use image::{DynamicImage, GenericImageView, Rgba};

use super::color_utils::ColorUtils;

#[derive(Debug)]
pub struct Pattern {
    coordinates: Vec<Coordinate>,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            coordinates: Vec::new(),
        }
    }

    pub fn from_image(image: DynamicImage, searched_color: &Rgba<u8>, tolerance: &u8) -> Self {
        let img_width = image.width();
        let img_height = image.height();

        let mut coordinates: Vec<Coordinate> = Vec::new();

        for y in 0..img_height {
            for x in 0..img_width {
                let pixel_color = &image.get_pixel(x, y);

                let is_equal =
                    ColorUtils::equal_with_tolerance(&searched_color, pixel_color, tolerance);

                if is_equal {
                    coordinates.push(Coordinate { x, y })
                }
            }
        }

        Pattern { coordinates }
    }
}

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

#[cfg(test)]
mod tests {
    use super::{Coordinate, Pattern};
    use image::{io::Reader as ImageReader, Rgba};

    #[test]
    fn test_from_image() {
        // Load the image from the specified path
        let image_path = "assets/images/crewmate.png";
        let image = ImageReader::open(image_path)
            .expect("Failed to open image")
            .decode()
            .expect("Failed to decode image");

        // Define the expected coordinates
        let expected_coordinates = vec![
            Coordinate { x: 1, y: 0 },
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 0 },
            Coordinate { x: 0, y: 1 },
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 0, y: 2 },
            Coordinate { x: 1, y: 2 },
            Coordinate { x: 2, y: 2 },
            Coordinate { x: 3, y: 2 },
            Coordinate { x: 1, y: 3 },
            Coordinate { x: 3, y: 3 },
        ];

        let pattern = Pattern::from_image(image, &Rgba([0, 0, 0, 0]), &0);

        assert_eq!(pattern.coordinates, expected_coordinates);
    }
}
