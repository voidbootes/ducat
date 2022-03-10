use crate::core::drawing::Canvas;
use crate::core::entity::definitions::Image;
use image::{GenericImage, ImageBuffer};
use std::i32;

#[rustfmt::skip]
pub fn draw_cross_mut<C>(canvas: &mut C, color: C::Pixel, x: i32, y: i32)
where
    C: Canvas
{
    let (width, height) = canvas.dimensions();
    let idx = |x, y| (3 * (y + 1) + x + 1) as usize;
    let stencil = [0u8, 1u8, 0u8,
                   1u8, 1u8, 1u8,
                   0u8, 1u8, 0u8];

    for sy in -1..2 {
        let iy = y + sy;
        if iy < 0 || iy >= height as i32 {
            continue;
        }

        for sx in -1..2 {
            let ix = x + sx;
            if ix < 0 || ix >= width as i32 {
                continue;
            }

            if stencil[idx(sx, sy)] == 1u8 {
                canvas.draw_pixel(ix as u32, iy as u32, color);
            }
        }
    }
}

pub fn draw_cross<I>(image: &I, color: I::Pixel, x: i32, y: i32) -> Image<I::Pixel>
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0).unwrap();
    draw_cross_mut(&mut out, color, x, y);
    out
}
