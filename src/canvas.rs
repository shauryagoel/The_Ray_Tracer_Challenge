use crate::Color;
use std::fs;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Canvas {
    // Initialize a new canvas of given dimensions with black pixels
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![Color::black(); width * height],
        }
    }

    // Fills the canvas with a new color
    pub fn fill(&mut self, color: Color) {
        self.data = vec![color; self.width * self.height];
    }

    pub fn write_pixel(&mut self, col: usize, row: usize, color: Color) {
        self[row][col] = color;
    }

    pub fn pixel_at(&self, col: usize, row: usize) -> Color {
        self[row][col]
    }

    // Convert canvas to ppm format
    pub fn get_ppm(&self) -> String {
        let header = self.get_ppm_header();
        let pixel_values = self.get_ppm_pixel_values();
        header + &pixel_values
    }

    // Write the string ppm to the `file_path`
    pub fn write_ppm(&self, ppm_string: &str, file_path: &str) {
        fs::write(file_path, ppm_string).expect("Unable to write ppm");
    }

    // PPM format-
    // PPM FLAVOUR (eg.- P3)
    // IMAGE_WIDTH IMAGE_HEIGHT (both are in pixels)
    // MAXIMUM_COLOR_VALUE (eg.- 255)
    fn get_ppm_header(&self) -> String {
        let mut header: String = String::from("");
        header += "P3\n";
        header += &(self.width.to_string() + " " + &self.height.to_string() + "\n");
        header += "255\n";
        header
    }

    fn get_ppm_pixel_values(&self) -> String {
        let mut pixels: String = String::from("");
        for row in 0..self.height {
            let mut pixels_row: String = String::from("");

            for col in 0..self.width {
                let _color = self.pixel_at(col, row);

                for pixel_value in [_color.red, _color.green, _color.blue] {
                    let pixel_value_string = self.scale_and_clip_color(pixel_value).to_string();

                    // Ensure that each row is at max 70 characters long
                    if pixels_row.len() + pixel_value_string.len() > 69 {
                        pixels_row.pop();
                        pixels_row += "\n";
                        pixels += &pixels_row;
                        pixels_row.clear();
                    }
                    pixels_row += &(pixel_value_string + " ");
                }
            }
            pixels += &pixels_row;
            // pixels = pixels.trim_end().to_string();
            pixels.pop(); // above might be slow as it creates a new string
            pixels += "\n";
        }
        pixels
    }

    // Scales the color value and clip between 0 and 255
    fn scale_and_clip_color(&self, color_value: f64) -> u8 {
        (color_value * 255_f64).clamp(0.0, 255.0).round() as u8
    }
}

// For indexing a row in a Canvas
impl std::ops::Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row * self.width..(row + 1) * self.width]
    }
}

// For indexing a row in a mutable Canvas
impl std::ops::IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row * self.width..(row + 1) * self.width]
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn canvas_init_with_black() {
        let c = Canvas::new(10, 20);
        let black_color = Color::new(0.0, 0.0, 0.0);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for i in 1..c.height {
            for j in 1..c.width {
                assert_eq!(c.pixel_at(j, i), black_color);
            }
        }
    }

    #[test]
    fn writing_a_pixel_in_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.get_ppm();
        let mut ppm_iter = ppm.split("\n");
        assert_eq!("P3", ppm_iter.next().unwrap());
        assert_eq!("5 3", ppm_iter.next().unwrap());
        assert_eq!("255", ppm_iter.next().unwrap());
    }

    #[test]
    fn ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.get_ppm();
        println!("{}", ppm);
        // Skips the header as we want to compare pixel data
        let mut ppm_iter = ppm.split("\n").skip(3);
        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", ppm_iter.next().unwrap());
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", ppm_iter.next().unwrap());
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", ppm_iter.next().unwrap());
    }

    #[test]
    fn ppm_row_length_check() {
        let mut c = Canvas::new(10, 2);
        c.fill(Color::new(1.0, 0.8, 0.6));
        let ppm = c.get_ppm();
        let mut ppm_iter = ppm.split("\n").skip(3);

        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            ppm_iter.next().unwrap()
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            ppm_iter.next().unwrap()
        );
        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            ppm_iter.next().unwrap()
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            ppm_iter.next().unwrap()
        );
    }

    #[test]
    fn ppm_newline_character_termination() {
        let c = Canvas::new(5, 3);
        let ppm = c.get_ppm();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
