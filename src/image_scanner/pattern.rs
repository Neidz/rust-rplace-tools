use image::{DynamicImage, GenericImageView, Rgba};

use super::{color_utils::ColorUtils, coordinate::Coordinate};

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

    pub fn new_from_coordinates(coordinates: Vec<Coordinate>) -> Self {
        Pattern { coordinates }
    }

    pub fn get_coordinates(&self) -> &Vec<Coordinate> {
        &self.coordinates
    }

    pub fn from_image(image: DynamicImage, search_color: Rgba<u8>, tolerance: u8) -> Self {
        let (img_width, img_height) = image.dimensions();

        let mut coordinates: Vec<Coordinate> = Vec::new();

        for y in 0..img_height as i32 {
            for x in 0..img_width as i32 {
                let pixel_color = image.get_pixel(x as u32, y as u32);

                let is_equal =
                    ColorUtils::equal_with_tolerance(search_color, pixel_color, tolerance);

                if is_equal {
                    coordinates.push(Coordinate::new(x, y))
                }
            }
        }

        Pattern { coordinates }
    }

    pub fn get_window_bounds(&self) -> (u32, u32) {
        let mut highest_x = 0;
        let mut highest_y = 0;

        for i in &self.coordinates {
            if i.x > highest_x {
                highest_x = i.x;
            }
            if i.y > highest_y {
                highest_y = i.y;
            }
        }

        (highest_x as u32 + 1, highest_y as u32 + 1)
    }

    pub fn contains_coordinate(&self, coordinate: &Coordinate) -> bool {
        self.coordinates.contains(coordinate)
    }
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

        let pattern = Pattern::from_image(image, Rgba([0, 0, 0, 0]), 0);

        assert_eq!(pattern.coordinates, expected_coordinates);
    }

    #[test]
    fn test_get_coordinates_bounds() {
        let pattern = Pattern {
            coordinates: vec![
                Coordinate { x: 2, y: 5 },
                Coordinate { x: 4, y: 3 },
                Coordinate { x: 1, y: 6 },
            ],
        };

        let expected_bounds = (5, 7);

        let actual_bounds = pattern.get_window_bounds();

        assert_eq!(actual_bounds, expected_bounds);
    }

    #[test]
    fn test_contains_coordinate() {
        let pattern = Pattern {
            coordinates: vec![
                Coordinate { x: 2, y: 5 },
                Coordinate { x: 4, y: 3 },
                Coordinate { x: 1, y: 6 },
            ],
        };

        assert!(pattern.contains_coordinate(&Coordinate { x: 2, y: 5 }));
        assert!(pattern.contains_coordinate(&Coordinate { x: 4, y: 3 }));
        assert!(pattern.contains_coordinate(&Coordinate { x: 1, y: 6 }));

        assert!(!pattern.contains_coordinate(&Coordinate { x: 0, y: 0 }));
        assert!(!pattern.contains_coordinate(&Coordinate { x: 3, y: 5 }));
        assert!(!pattern.contains_coordinate(&Coordinate { x: 1, y: 1 }));
    }
}
