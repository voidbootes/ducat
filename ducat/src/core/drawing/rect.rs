use crate::core::drawing::line::draw_line_segment_mut;
use crate::core::drawing::Canvas;
use crate::core::entity::definitions::Image;
use crate::core::entity::rect::Rect;
use image::{GenericImage, ImageBuffer};
use std::f32;

pub fn draw_hollow_rect<I>(image: &I, rect: Rect, color: I::Pixel) -> Image<I::Pixel>
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0).unwrap();
    draw_hollow_rect_mut(&mut out, rect, color);
    out
}

pub fn draw_hollow_rect_mut<C>(canvas: &mut C, rect: Rect, color: C::Pixel)
where
    C: Canvas,
    C::Pixel: 'static,
{
    let left = rect.left() as f32;
    let right = rect.right() as f32;
    let top = rect.top() as f32;
    let bottom = rect.bottom() as f32;

    draw_line_segment_mut(canvas, (left, top), (right, top), color);
    draw_line_segment_mut(canvas, (left, bottom), (right, bottom), color);
    draw_line_segment_mut(canvas, (left, top), (left, bottom), color);
    draw_line_segment_mut(canvas, (right, top), (right, bottom), color);
}

pub fn draw_filled_rect<I>(image: &I, rect: Rect, color: I::Pixel) -> Image<I::Pixel>
where
    I: GenericImage,
    I::Pixel: 'static,
{
    let mut out = ImageBuffer::new(image.width(), image.height());
    out.copy_from(image, 0, 0).unwrap();
    draw_filled_rect_mut(&mut out, rect, color);
    out
}

pub fn draw_filled_rect_mut<C>(canvas: &mut C, rect: Rect, color: C::Pixel)
where
    C: Canvas,
    C::Pixel: 'static,
{
    let canvas_bounds = Rect::at(0, 0).of_size(canvas.width(), canvas.height());
    if let Some(intersection) = canvas_bounds.intersect(rect) {
        for dy in 0..intersection.height() {
            for dx in 0..intersection.width() {
                let x = intersection.left() as u32 + dx;
                let y = intersection.top() as u32 + dy;
                canvas.draw_pixel(x, y, color);
            }
        }
    }
}
