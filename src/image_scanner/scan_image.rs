use std::sync::Arc;

use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;

use super::{
    color_utils::ColorUtils, coordinate::Coordinate, image_utils::ImageUtils, pattern::Pattern,
};

pub fn scan_image(
    image: &DynamicImage,
    search_pattern: &Pattern,
    pattern_search_tolerance: u8,
) -> Vec<Pattern> {
    let (img_width, img_height) = image.dimensions();
    let (window_width, window_height) = search_pattern.get_window_size();

    let adjacent_pixel_coordinates =
        Arc::new(search_pattern.generate_coordinates_of_adjacent_pixels());

    let found_patterns: Vec<Pattern> = (0..(img_height - window_height))
        .into_par_iter()
        .flat_map(|offset_y| {
            let adjacent_pixel_coordinates = Arc::clone(&adjacent_pixel_coordinates);

            (0..(img_width - window_width))
                .into_par_iter()
                .filter_map(move |offset_x| {
                    pattern_in_window(
                        image,
                        offset_x,
                        offset_y,
                        search_pattern,
                        pattern_search_tolerance,
                        &adjacent_pixel_coordinates,
                    )
                })
        })
        .collect();

    found_patterns
}

fn pattern_in_window(
    image: &DynamicImage,
    offset_x: u32,
    offset_y: u32,
    search_pattern: &Pattern,
    pattern_search_tolerance: u8,
    adjacent_pixel_coordinates: &Arc<Vec<Coordinate>>,
) -> Option<Pattern> {
    let (window_width, window_height) = search_pattern.get_window_size();
    let window = image.view(offset_x, offset_y, window_width, window_height);
    let pattern_coordinates = search_pattern.get_coordinates();

    let first_pixel_color = window.get_pixel(
        pattern_coordinates[0].x as u32,
        pattern_coordinates[0].y as u32,
    );

    for coordinate in pattern_coordinates {
        let pixel_color = window.get_pixel(coordinate.x as u32, coordinate.y as u32);

        if !ColorUtils::equal_with_tolerance(
            first_pixel_color,
            pixel_color,
            pattern_search_tolerance,
        ) {
            return None;
        }
    }

    let window_with_border =
        ImageUtils::create_view_with_border(image, offset_x, offset_y, window_width, window_height);
    let (window_with_border_width, window_with_border_height) = {
        let (width, height) = window_with_border.dimensions();
        (width as i32, height as i32)
    };

    let expanded_x_by = if offset_x == 0 { 0 } else { 1 };
    let expanded_y_by = if offset_y == 0 { 0 } else { 1 };

    for coordinate in adjacent_pixel_coordinates.iter() {
        let adjusted_x = coordinate.x + expanded_x_by;
        let adjusted_y = coordinate.y + expanded_y_by;

        if adjusted_x < window_with_border_width
            && adjusted_y < window_with_border_height
            && adjusted_x >= 0
            && adjusted_y >= 0
        {
            let adjacent_pixel_color =
                window_with_border.get_pixel(adjusted_x as u32, adjusted_y as u32);

            if ColorUtils::equal_with_tolerance(
                first_pixel_color,
                adjacent_pixel_color,
                pattern_search_tolerance,
            ) {
                return None;
            }
        }
    }

    let coordinates_of_found_pattern = pattern_coordinates
        .iter()
        .map(|coord| Coordinate {
            x: coord.x + offset_x as i32,
            y: coord.y + offset_y as i32,
        })
        .collect();

    Some(Pattern::new_from_coordinates(coordinates_of_found_pattern))
}

#[cfg(test)]
mod tests {
    use crate::image_scanner::pattern::Pattern;

    use super::scan_image;
    use image::io::Reader as ImageReader;
    use image::Rgba;

    fn load_image(image_path: &str) -> image::DynamicImage {
        ImageReader::open(image_path)
            .expect("Failed to open image")
            .decode()
            .expect("Failed to decode image")
    }

    const SEARCHED_COLOR: Rgba<u8> = Rgba([0, 0, 0, 0]);
    const PATTERN_EXTRACTING_TOLERANCE: u8 = 1;
    const PATTERN_SEARCHING_TOLERANCE: u8 = 1;

    #[test]
    fn test_scan_image_simple() {
        let pattern_image = load_image("assets/images/crewmate.png");
        let scanned_image = load_image("assets/images/crewmate_with_borders.png");

        let search_pattern =
            Pattern::from_image(pattern_image, SEARCHED_COLOR, PATTERN_EXTRACTING_TOLERANCE);

        let found_patterns =
            scan_image(&scanned_image, &search_pattern, PATTERN_SEARCHING_TOLERANCE);

        assert_eq!(found_patterns.len(), 1)
    }

    #[test]
    fn test_scan_image_different_colors() {
        let pattern_image = load_image("assets/images/crewmate.png");
        let scanned_image = load_image("assets/images/8_crewmates.png");

        let search_pattern =
            Pattern::from_image(pattern_image, SEARCHED_COLOR, PATTERN_EXTRACTING_TOLERANCE);

        let found_patterns =
            scan_image(&scanned_image, &search_pattern, PATTERN_SEARCHING_TOLERANCE);

        assert_eq!(found_patterns.len(), 8)
    }

    #[test]
    fn test_scan_image_adjacent() {
        let pattern_image = load_image("assets/images/crewmate.png");
        let scanned_image = load_image("assets/images/4_crewmates_adjacent_test.png");

        let search_pattern =
            Pattern::from_image(pattern_image, SEARCHED_COLOR, PATTERN_EXTRACTING_TOLERANCE);

        let found_patterns =
            scan_image(&scanned_image, &search_pattern, PATTERN_SEARCHING_TOLERANCE);

        assert_eq!(found_patterns.len(), 4)
    }

    #[test]
    fn test_scan_image_different_adjacent_2() {
        let pattern_image = load_image("assets/images/crewmate.png");
        let scanned_image = load_image("assets/images/4_crewmates_adjacent_test_2.png");

        let search_pattern =
            Pattern::from_image(pattern_image, SEARCHED_COLOR, PATTERN_EXTRACTING_TOLERANCE);

        let found_patterns =
            scan_image(&scanned_image, &search_pattern, PATTERN_SEARCHING_TOLERANCE);

        assert_eq!(found_patterns.len(), 4)
    }
}
