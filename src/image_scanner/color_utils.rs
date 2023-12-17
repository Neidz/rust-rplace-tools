use image::Rgba;

pub struct ColorUtils;

impl ColorUtils {
    pub fn equal_with_tolerance(color1: &Rgba<u8>, color2: &Rgba<u8>, tolerance: &u8) -> bool {
        let delta_r = (color1.0[0] as i16 - color2.0[0] as i16).abs() as u8;
        let delta_g = (color1.0[1] as i16 - color2.0[1] as i16).abs() as u8;
        let delta_b = (color1.0[2] as i16 - color2.0[2] as i16).abs() as u8;

        delta_r <= *tolerance && delta_g <= *tolerance && delta_b <= *tolerance
    }
}

#[cfg(test)]
mod tests {
    use image::Rgba;

    use super::ColorUtils;

    #[test]
    fn test_equal_with_tolerance_exact_match() {
        let color1 = Rgba([255, 0, 0, 255]);
        let color2 = Rgba([255, 0, 0, 255]);
        let tolerance = 0;

        assert!(ColorUtils::equal_with_tolerance(
            &color1, &color2, &tolerance
        ));
    }

    #[test]
    fn test_equal_with_tolerance_within_tolerance() {
        let color1 = Rgba([255, 0, 0, 255]);
        let color2 = Rgba([250, 5, 0, 255]);
        let tolerance = 10;

        assert!(ColorUtils::equal_with_tolerance(
            &color1, &color2, &tolerance
        ));
    }

    #[test]
    fn test_equal_with_tolerance_outside_tolerance() {
        let color1 = Rgba([255, 0, 0, 255]);
        let color2 = Rgba([200, 50, 0, 255]);
        let tolerance = 10;

        assert!(!ColorUtils::equal_with_tolerance(
            &color1, &color2, &tolerance
        ));
    }
}
