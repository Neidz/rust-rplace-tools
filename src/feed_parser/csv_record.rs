use chrono::NaiveDateTime;
use image::{DynamicImage, GenericImage, Rgba};
use serde::de;
use serde_derive;

#[derive(Debug, serde_derive::Deserialize)]
pub struct CSVRecord {
    #[serde(deserialize_with = "deserialize_timestamp")]
    timestamp: NaiveDateTime,
    user: String,
    #[serde(deserialize_with = "deserialize_coordinate")]
    coordinate: Coordinate,
    #[serde(deserialize_with = "deserialize_color")]
    pixel_color: Rgba<u8>,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp_str: &str = serde::Deserialize::deserialize(deserializer)?;

    NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.f").map_err(de::Error::custom)
}

fn deserialize_coordinate<'de, D>(deserializer: D) -> Result<Coordinate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let coordinate_str: &str = serde::Deserialize::deserialize(deserializer)?;

    let cleaned_coordinate_str = coordinate_str.trim_matches('"');

    let parts: Vec<i32> = cleaned_coordinate_str
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<_, _>>()
        .map_err(|_| de::Error::custom("Failed to parse coordinate"))?;

    match parts.len() {
        2 => Ok(Coordinate::Point {
            x: parts[0],
            y: parts[1],
        }),
        3 => Ok(Coordinate::Circle {
            x: parts[0],
            y: parts[1],
            r: parts[2],
        }),
        4 => Ok(Coordinate::Rectangle {
            x1: parts[0],
            x2: parts[1],
            y1: parts[2],
            y2: parts[3],
        }),
        _ => Err(de::Error::custom("Invalid coordinate format")),
    }
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Rgba<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let color_str: &str = serde::Deserialize::deserialize(deserializer)?;

    if let Ok(color) = parse_color_from_hex(color_str) {
        Ok(color)
    } else {
        Err(de::Error::custom(format!(
            "Failed to parse color from hex: {}",
            color_str
        )))
    }
}

fn parse_color_from_hex(color_str: &str) -> Result<Rgba<u8>, &'static str> {
    if color_str.len() == 7 && color_str.starts_with('#') {
        let r = u8::from_str_radix(&color_str[1..3], 16).map_err(|_| "Invalid red component")?;
        let g = u8::from_str_radix(&color_str[3..5], 16).map_err(|_| "Invalid green component")?;
        let b = u8::from_str_radix(&color_str[5..7], 16).map_err(|_| "Invalid blue component")?;

        Ok(Rgba([r, g, b, 255]))
    } else {
        Err("Invalid hex color format")
    }
}

impl CSVRecord {
    fn draw_on_image(&self, image: &mut DynamicImage) {
        match &self.coordinate {
            Coordinate::Point { x, y } => {
                image.put_pixel(*x as u32, *y as u32, self.pixel_color);
            }
            Coordinate::Circle { x, y, r } => {
                todo!()
            }
            Coordinate::Rectangle { x1, x2, y1, y2 } => {
                todo!()
            }
        }
    }
}

#[derive(Debug, serde_derive::Deserialize, PartialEq)]
#[serde(untagged)]
enum Coordinate {
    Point { x: i32, y: i32 },
    Circle { x: i32, y: i32, r: i32 },
    Rectangle { x1: i32, x2: i32, y1: i32, y2: i32 },
}

#[cfg(test)]
mod csv_parsing_tests {
    use super::{CSVRecord, Coordinate};
    use chrono::NaiveDateTime;
    use csv::ReaderBuilder;
    use image::Rgba;
    use std::io::Cursor;

    #[test]
    fn test_parse_csv_record() {
        let csv_data = "timestamp,user,coordinate,pixel_color
        2023-07-20 13:00:26.088,no+8HEIDjbdx7/LxH9Xr+h4lyoar0MRTYugWKrGdQOg7dFg0rU9STehlIqsje1kc48U/BQqB/0J8sHQzXJBDFA==,\"-199,-235\",#FFFFFF";

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_data));

        let record: CSVRecord = reader.deserialize().next().unwrap().unwrap();

        let expected_timestamp =
            NaiveDateTime::parse_from_str("2023-07-20 13:00:26.088", "%Y-%m-%d %H:%M:%S%.f")
                .unwrap();
        let expected_user = String::from("no+8HEIDjbdx7/LxH9Xr+h4lyoar0MRTYugWKrGdQOg7dFg0rU9STehlIqsje1kc48U/BQqB/0J8sHQzXJBDFA==");
        let expected_pixel_color = Rgba([255, 255, 255, 255]);

        assert_eq!(record.timestamp, expected_timestamp);
        assert_eq!(record.user, expected_user);
        assert_eq!(record.pixel_color, expected_pixel_color);
    }

    #[test]
    fn test_parse_point_coordinate() {
        let csv_data = "timestamp,user,coordinate,pixel_color
        2023-07-20 13:00:26.088,user,\"-199,-235\",#FFFFFF";

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_data));

        let record: CSVRecord = reader.deserialize().next().unwrap().unwrap();
        let expected_coordinate = Coordinate::Point { x: -199, y: -235 };

        assert_eq!(record.coordinate, expected_coordinate);
    }

    #[test]
    fn test_parse_circle_coordinate() {
        let csv_data = "timestamp,user,coordinate,pixel_color
        2023-07-20 13:00:26.088,user,\"-199,-235,10\",#FFFFFF";

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_data));

        let record: CSVRecord = reader.deserialize().next().unwrap().unwrap();
        let expected_coordinate = Coordinate::Circle {
            x: -199,
            y: -235,
            r: 10,
        };

        assert_eq!(record.coordinate, expected_coordinate);
    }

    #[test]
    fn test_parse_rectangle_coordinate() {
        let csv_data = "timestamp,user,coordinate,pixel_color
        2023-07-20 13:00:26.088,user,\"-199,100,-235,150\",#FFFFFF";

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_data));

        let record: CSVRecord = reader.deserialize().next().unwrap().unwrap();
        let expected_coordinate = Coordinate::Rectangle {
            x1: -199,
            x2: 100,
            y1: -235,
            y2: 150,
        };

        assert_eq!(record.coordinate, expected_coordinate);
    }
}

#[cfg(test)]
mod hex_parsing_tests {
    use image::Rgba;

    use super::parse_color_from_hex;

    #[test]
    fn test_valid_hex_color() {
        assert_eq!(
            parse_color_from_hex("#AABBCC"),
            Ok(Rgba([170, 187, 204, 255]))
        );
    }

    #[test]
    fn test_another_valid_hex_color() {
        assert_eq!(parse_color_from_hex("#112233"), Ok(Rgba([17, 34, 51, 255])));
    }

    #[test]
    fn test_invalid_format() {
        assert_eq!(
            parse_color_from_hex("invalid"),
            Err("Invalid hex color format")
        );
    }

    #[test]
    fn test_invalid_length() {
        assert_eq!(
            parse_color_from_hex("#12345"),
            Err("Invalid hex color format")
        );
    }

    #[test]
    fn test_invalid_alpha_component() {
        assert_eq!(
            parse_color_from_hex("#AABBCCDD"),
            Err("Invalid hex color format")
        );
    }
}
