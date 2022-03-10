use crate::core::entity::definitions::Clamp;
use crate::core::operation::math::cast;
use conv::ValueInto;
use image::Pixel;

pub fn weighted_sum<P: Pixel>(left: P, right: P, left_weight: f32, right_weight: f32) -> P
where
    P::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    left.map2(&right, |p, q| {
        weighted_channel_sum(p, q, left_weight, right_weight)
    })
}

pub fn interpolate<P: Pixel>(left: P, right: P, left_weight: f32) -> P
where
    P::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    weighted_sum(left, right, left_weight, 1.0 - left_weight)
}

#[inline(always)]
fn weighted_channel_sum<C>(left: C, right: C, left_weight: f32, right_weight: f32) -> C
where
    C: ValueInto<f32> + Clamp<f32>,
{
    Clamp::clamp(cast(left) * left_weight + cast(right) * right_weight)
}
