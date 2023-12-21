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
    let pattern_coordinates = search_pattern.get_coordinates();
    let (window_width, window_height) = search_pattern.get_window_size();

    let found_patterns: Vec<Pattern> = (0..(img_height - window_height))
        .into_par_iter()
        .flat_map(|offset_y| {
            (0..(img_width - window_width))
                .into_par_iter()
                .filter_map(move |offset_x| {
                    if is_pattern_in_window(
                        image,
                        offset_x,
                        offset_y,
                        search_pattern,
                        pattern_search_tolerance,
                    ) {
                        let coordinates_of_found_pattern = pattern_coordinates
                            .iter()
                            .map(|coord| Coordinate {
                                x: coord.x + offset_x as i32,
                                y: coord.y + offset_y as i32,
                            })
                            .collect();

                        Some(Pattern::new_from_coordinates(coordinates_of_found_pattern))
                    } else {
                        None
                    }
                })
        })
        .collect();

    found_patterns
}

fn is_pattern_in_window(
    image: &DynamicImage,
    offset_x: u32,
    offset_y: u32,
    search_pattern: &Pattern,
    pattern_search_tolerance: u8,
) -> bool {
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
            return false;
        }
    }

    let adjacent_pixel_coordinates = search_pattern.generate_coordinates_of_adjacent_pixels();

    let window_with_border =
        ImageUtils::create_view_with_border(image, offset_x, offset_y, window_width, window_height);
    let (window_with_border_width, window_with_border_height) = {
        let (width, height) = window_with_border.dimensions();
        (width as i32, height as i32)
    };

    let expanded_x_by = if offset_x == 0 { 0 } else { 1 };
    let expanded_y_by = if offset_y == 0 { 0 } else { 1 };

    for coordinate in adjacent_pixel_coordinates {
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
                return false;
            }
        }
    }

    return true;
}
