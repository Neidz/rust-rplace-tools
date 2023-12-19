use image::{DynamicImage, GenericImageView, Rgba};

use super::{
    color_utils::ColorUtils,
    pattern::{Coordinate, Pattern},
};

pub fn scan_image(
    image: &DynamicImage,
    search_pattern: &Pattern,
    pattern_search_tolerance: u8,
) -> Vec<Pattern> {
    let (img_width, img_height) = image.dimensions();

    let mut found_patterns: Vec<Pattern> = Vec::new();

    let (searched_window_width, searched_window_height) = &search_pattern.get_window_bounds();

    for offset_y in 0..img_height - searched_window_height {
        for offset_x in 0..img_width - searched_window_width {
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
    img: &DynamicImage,
    search_pattern: &Pattern,
    offset_x: u32,
    offset_y: u32,
    pattern_search_tolerance: u8,
) -> Option<Pattern> {
    let pattern_coordinates = search_pattern.get_coordinates();

    let first_pixel_color = *&img.get_pixel(
        pattern_coordinates[0].x + offset_x,
        pattern_coordinates[0].y + offset_y,
    );

    for coordinate in pattern_coordinates {
        let coordinate_with_offset = Coordinate {
            x: coordinate.x + offset_x,
            y: coordinate.y + offset_y,
        };

        let pixel_color = get_pixel_safe(img, coordinate_with_offset.x, coordinate_with_offset.y)?;

        if !ColorUtils::equal_with_tolerance(
            first_pixel_color,
            pixel_color,
            pattern_search_tolerance,
        ) {
            return None;
        }

        for surrounding_offset in SURROUNDING_OFFSETS {
            let (surrounding_x, surrounding_y) = (
                coordinate_with_offset.x as i32 + surrounding_offset.0,
                coordinate_with_offset.y as i32 + surrounding_offset.1,
            );

            let surrounding_pixel_color =
                get_pixel_safe_unsigned(img, surrounding_x, surrounding_y)?;

            let same_color = ColorUtils::equal_with_tolerance(
                first_pixel_color,
                surrounding_pixel_color,
                pattern_search_tolerance,
            );

            if offset_x <= surrounding_x as u32 && offset_y <= surrounding_y as u32 {
                let in_pattern = search_pattern.contains_coordinate(&Coordinate {
                    x: surrounding_x as u32 - offset_x,
                    y: surrounding_y as u32 - offset_y,
                });

                if !in_pattern && same_color {
                    return None;
                }
            }
        }
    }

    let coordinates_for_found_pattern = pattern_coordinates
        .iter()
        .map(|coord| Coordinate {
            x: coord.x + offset_x,
            y: coord.y + offset_y,
        })
        .collect();

    Some(Pattern::new_from_coordinates(coordinates_for_found_pattern))
}

fn get_pixel_safe(image: &DynamicImage, x: u32, y: u32) -> Option<Rgba<u8>> {
    let (width, height) = image.dimensions();

    if x >= width || y >= height {
        return None;
    }

    return Some(image.get_pixel(x, y));
}

fn get_pixel_safe_unsigned(image: &DynamicImage, x: i32, y: i32) -> Option<Rgba<u8>> {
    let (width, height) = image.dimensions();

    if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
        return None;
    }

    Some(image.get_pixel(x as u32, y as u32))
}

const SURROUNDING_OFFSETS: [(i32, i32); 8] = [
    (LEFT, TOP),
    (CENTER, TOP),
    (RIGHT, TOP),
    (LEFT, CENTER),
    (RIGHT, CENTER),
    (LEFT, BOTTOM),
    (CENTER, BOTTOM),
    (RIGHT, BOTTOM),
];

const LEFT: i32 = -1;
const RIGHT: i32 = 1;
const TOP: i32 = -1;
const BOTTOM: i32 = 1;
const CENTER: i32 = 0;
