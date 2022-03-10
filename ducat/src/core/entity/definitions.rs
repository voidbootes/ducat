use image::{Bgr, Bgra, ImageBuffer, Luma, LumaA, Pixel, Rgb, Rgba};
use std::{i16, u16, u8};

pub type Image<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;

pub trait HasBlack {
    fn black() -> Self;
}

pub trait HasWhite {
    fn white() -> Self;
}

macro_rules! impl_black_white {
    ($for_:ty, $min:expr, $max:expr) => {
        impl HasBlack for $for_ {
            fn black() -> Self {
                $min
            }
        }

        impl HasWhite for $for_ {
            fn white() -> Self {
                $max
            }
        }
    };
}

impl_black_white!(Luma<u8>, Luma([u8::MIN]), Luma([u8::MAX]));
impl_black_white!(Luma<u16>, Luma([u16::MIN]), Luma([u16::MAX]));
impl_black_white!(
    LumaA<u8>,
    LumaA([u8::MIN, u8::MAX]),
    LumaA([u8::MAX, u8::MAX])
);
impl_black_white!(
    LumaA<u16>,
    LumaA([u16::MIN, u16::MAX]),
    LumaA([u16::MAX, u16::MAX])
);
impl_black_white!(Rgb<u8>, Rgb([u8::MIN; 3]), Rgb([u8::MAX; 3]));
impl_black_white!(Rgb<u16>, Rgb([u16::MIN; 3]), Rgb([u16::MAX; 3]));
impl_black_white!(
    Rgba<u8>,
    Rgba([u8::MIN, u8::MIN, u8::MIN, u8::MAX]),
    Rgba([u8::MAX, u8::MAX, u8::MAX, u8::MAX])
);
impl_black_white!(
    Rgba<u16>,
    Rgba([u16::MIN, u16::MIN, u16::MIN, u16::MAX]),
    Rgba([u16::MAX, u16::MAX, u16::MAX, u16::MAX])
);
impl_black_white!(Bgr<u8>, Bgr([u8::MIN; 3]), Bgr([u8::MAX; 3]));
impl_black_white!(Bgr<u16>, Bgr([u16::MIN; 3]), Bgr([u16::MAX; 3]));
impl_black_white!(
    Bgra<u8>,
    Bgra([u8::MIN, u8::MIN, u8::MIN, u8::MAX]),
    Bgra([u8::MAX, u8::MAX, u8::MAX, u8::MAX])
);
impl_black_white!(
    Bgra<u16>,
    Bgra([u16::MIN, u16::MIN, u16::MIN, u16::MAX]),
    Bgra([u16::MAX, u16::MAX, u16::MAX, u16::MAX])
);

pub trait Position {
    fn x(&self) -> u32;
    fn y(&self) -> u32;
}

pub trait Score {
    fn score(&self) -> f32;
}

pub trait Clamp<T> {
    fn clamp(x: T) -> Self;
}

macro_rules! implement_clamp {
    ($from:ty, $to:ty, $min:expr, $max:expr, $min_from:expr, $max_from:expr) => {
        impl Clamp<$from> for $to {
            fn clamp(x: $from) -> $to {
                if x < $max_from as $from {
                    if x > $min_from as $from {
                        x as $to
                    } else {
                        $min
                    }
                } else {
                    $max
                }
            }
        }
    };
}

macro_rules! implement_identity_clamp {
    ( $($t:ty),* ) => {
        $(
            impl Clamp<$t> for $t {
                fn clamp(x: $t) -> $t { x }
            }
        )*
    };
}

implement_clamp!(i16, u8, u8::MIN, u8::MAX, u8::MIN as i16, u8::MAX as i16);
implement_clamp!(u16, u8, u8::MIN, u8::MAX, u8::MIN as u16, u8::MAX as u16);
implement_clamp!(i32, u8, u8::MIN, u8::MAX, u8::MIN as i32, u8::MAX as i32);
implement_clamp!(u32, u8, u8::MIN, u8::MAX, u8::MIN as u32, u8::MAX as u32);
implement_clamp!(f32, u8, u8::MIN, u8::MAX, u8::MIN as f32, u8::MAX as f32);
implement_clamp!(f64, u8, u8::MIN, u8::MAX, u8::MIN as f64, u8::MAX as f64);

implement_clamp!(
    i32,
    u16,
    u16::MIN,
    u16::MAX,
    u16::MIN as i32,
    u16::MAX as i32
);
implement_clamp!(
    f32,
    u16,
    u16::MIN,
    u16::MAX,
    u16::MIN as f32,
    u16::MAX as f32
);
implement_clamp!(
    f64,
    u16,
    u16::MIN,
    u16::MAX,
    u16::MIN as f64,
    u16::MAX as f64
);

implement_clamp!(
    i32,
    i16,
    i16::MIN,
    i16::MAX,
    i16::MIN as i32,
    i16::MAX as i32
);

implement_identity_clamp!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
