use image::{ImageBuffer, Rgba};
use std::env;
use std::path::Path;

use ducat::core::drawing::draw_line_segment_mut;

fn main() {
    let arg = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a target file path")
    };

    let path = Path::new(&arg);

    let color = Rgba([97u8, 142u8, 207u8, 255u8]);
    let mut image = ImageBuffer::from_pixel(1600, 1600, Rgba([255u8, 255u8, 255u8, 255u8]));

    let start = (0.0, 0.0);
    let end = (100.0, 100.0);
    draw_line_segment_mut(&mut image, start, end, color);

    image.save(path).unwrap();
}
