use image::{DynamicImage, GenericImageView, Rgba};

use super::{color_utils::ColorUtils, coordinate::Coordinate, pattern::Pattern};

pub fn scan_image(
    image: &DynamicImage,
    search_pattern: &Pattern,
    pattern_search_tolerance: u8,
) -> Vec<Pattern> {
    let (img_width, img_height) = image.dimensions();

    let mut found_patterns: Vec<Pattern> = Vec::new();

    let (window_width, window_height) = &search_pattern.get_window_bounds();

    for offset_y in 0..img_height - window_height {
        for offset_x in 0..img_width - window_width {
            if let Some(pattern) = search_for_pattern_in_window(
                &image,
                &search_pattern,
                offset_x,
                offset_y,
                pattern_search_tolerance,
            ) {
                found_patterns.push(pattern);
            }
        }
    }

    found_patterns
}

fn search_for_pattern_in_window(
    image: &DynamicImage,
    search_pattern: &Pattern,
    offset_x: u32,
    offset_y: u32,
    pattern_search_tolerance: u8,
) -> Option<Pattern> {
    let pattern_coordinates = search_pattern.get_coordinates();

    let first_pixel_color = *&image.get_pixel(
        pattern_coordinates[0].x as u32 + offset_x,
        pattern_coordinates[0].y as u32 + offset_y,
    );

    for coordinate in pattern_coordinates {
        let coordinate_with_offset = Coordinate::new(
            coordinate.x + offset_x as i32,
            coordinate.y + offset_y as i32,
        );

        let pixel_color =
            get_pixel_safe(image, coordinate_with_offset.x, coordinate_with_offset.y)?;

        if !ColorUtils::equal_with_tolerance(
            first_pixel_color,
            pixel_color,
            pattern_search_tolerance,
        ) {
            return None;
        }

        if !are_surrounding_pixels_valid(
            image,
            search_pattern,
            &coordinate_with_offset,
            pattern_search_tolerance,
            offset_x,
            offset_y,
            first_pixel_color,
        ) {
            return None;
        }
    }

    let coordinates_for_found_pattern = pattern_coordinates
        .iter()
        .map(|coord| Coordinate {
            x: coord.x + offset_x as i32,
            y: coord.y + offset_y as i32,
        })
        .collect();

    Some(Pattern::new_from_coordinates(coordinates_for_found_pattern))
}

fn are_surrounding_pixels_valid(
    image: &DynamicImage,
    search_pattern: &Pattern,
    coordinate_with_offset: &Coordinate,
    pattern_search_tolerance: u8,
    offset_x: u32,
    offset_y: u32,
    first_pixel_color: Rgba<u8>,
) -> bool {
    for surrounding_offset in SURROUNDING_OFFSETS {
        let surrounding_coordinate = Coordinate::new(
            coordinate_with_offset.x + surrounding_offset.x,
            coordinate_with_offset.y + surrounding_offset.y,
        );

        if let Some(surrounding_pixel_color) =
            get_pixel_safe(image, surrounding_coordinate.x, surrounding_coordinate.y)
        {
            let same_color = ColorUtils::equal_with_tolerance(
                first_pixel_color,
                surrounding_pixel_color,
                pattern_search_tolerance,
            );

            if offset_x <= surrounding_coordinate.x as u32
                && offset_y <= surrounding_coordinate.y as u32
            {
                let in_pattern = search_pattern.contains_coordinate(&Coordinate::new(
                    surrounding_coordinate.x - offset_x as i32,
                    surrounding_coordinate.y - offset_y as i32,
                ));

                if !in_pattern && same_color {
                    return false;
                }
            }
        } else {
            return false;
        };
    }

    true
}

fn get_pixel_safe(image: &DynamicImage, x: i32, y: i32) -> Option<Rgba<u8>> {
    let (width, height) = image.dimensions();

    if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
        return None;
    }

    Some(image.get_pixel(x as u32, y as u32))
}

const SURROUNDING_OFFSETS: [Coordinate; 8] = [
    Coordinate { x: LEFT, y: TOP },
    Coordinate { x: CENTER, y: TOP },
    Coordinate { x: RIGHT, y: TOP },
    Coordinate { x: LEFT, y: CENTER },
    Coordinate {
        x: RIGHT,
        y: CENTER,
    },
    Coordinate { x: LEFT, y: BOTTOM },
    Coordinate {
        x: CENTER,
        y: BOTTOM,
    },
    Coordinate {
        x: RIGHT,
        y: BOTTOM,
    },
];

const LEFT: i32 = -1;
const RIGHT: i32 = 1;
const TOP: i32 = -1;
const BOTTOM: i32 = 1;
const CENTER: i32 = 0;

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageBuffer, Rgba};

    #[test]
    fn test_get_pixel_safe_within_bounds() {
        let image: DynamicImage =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(10, 10, Rgba([0, 10, 20, 255])));

        let result = get_pixel_safe(&image, 5, 5);
        assert_eq!(result, Some(Rgba([0, 10, 20, 255])));
    }

    #[test]
    fn test_get_pixel_safe_out_of_bounds() {
        let image: DynamicImage =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(10, 10, Rgba([0, 10, 20, 255])));

        let result = get_pixel_safe(&image, 15, 15);
        assert_eq!(result, None);
    }
}
