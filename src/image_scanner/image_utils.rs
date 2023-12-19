use image::{DynamicImage, GenericImageView, SubImage};

pub struct ImageUtils {}

impl ImageUtils {
    pub fn create_view_with_border(
        image: &DynamicImage,
        offset_x: u32,
        offset_y: u32,
        window_width: u32,
        window_height: u32,
    ) -> SubImage<&DynamicImage> {
        let border_x_start = offset_x.saturating_sub(1);
        let border_y_start = offset_y.saturating_sub(1);

        let border_x_end = (border_x_start + window_width + 2).min(image.width());
        let border_y_end = (border_y_start + window_height + 2).min(image.height());

        image.view(
            border_x_start,
            border_y_start,
            border_x_end - border_x_start,
            border_y_end - border_y_start,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ImageUtils;
    use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

    #[test]
    fn test_create_view_with_border_within_bounds() {
        let image: DynamicImage =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(10, 10, Rgba([0, 10, 20, 255])));

        let result = ImageUtils::create_view_with_border(&image, 2, 2, 5, 5);

        assert_eq!(result.width(), 7);
        assert_eq!(result.height(), 7);
    }

    #[test]
    fn test_create_view_with_border_at_image_bounds() {
        let image: DynamicImage =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(10, 10, Rgba([0, 10, 20, 255])));

        let result = ImageUtils::create_view_with_border(&image, 8, 8, 5, 5);

        assert_eq!(result.width(), 3);
        assert_eq!(result.height(), 3);
    }

    #[test]
    fn test_create_view_with_border_negative_offset() {
        let image: DynamicImage =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(10, 10, Rgba([0, 10, 20, 255])));

        let result = ImageUtils::create_view_with_border(&image, 2, 2, 5, 5);

        assert_eq!(result.width(), 7);
        assert_eq!(result.height(), 7);
    }
}
