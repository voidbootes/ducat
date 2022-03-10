use std::cmp;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rect {
    left: i32,
    top: i32,
    width: u32,
    height: u32,
}

pub trait Region<T> {
    fn contains(&self, x: T, y: T) -> bool;
}

impl Rect {
    pub fn at(x: i32, y: i32) -> RectPosition {
        RectPosition { left: x, top: y }
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn bottom(&self) -> i32 {
        self.top + (self.height as i32) - 1
    }

    pub fn right(&self) -> i32 {
        self.left + (self.width as i32) - 1
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn intersect(&self, other: Rect) -> Option<Rect> {
        let left = cmp::max(self.left, other.left);
        let top = cmp::max(self.top, other.top);
        let right = cmp::min(self.right(), other.right());
        let bottom = cmp::min(self.bottom(), other.bottom());

        if right < left || bottom < top {
            return None;
        }

        Some(Rect {
            left,
            top,
            width: (right - left) as u32 + 1,
            height: (bottom - top) as u32 + 1,
        })
    }
}

impl Region<i32> for Rect {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.left <= x && x <= self.right() && self.top <= y && y <= self.bottom()
    }
}

impl Region<f32> for Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        self.left as f32 <= x
            && x <= self.right() as f32
            && self.top as f32 <= y
            && y <= self.bottom() as f32
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RectPosition {
    left: i32,
    top: i32,
}

impl RectPosition {
    pub fn of_size(self, width: u32, height: u32) -> Rect {
        assert!(width > 0, "width must be strictly positive");
        assert!(height > 0, "height must be strictly positive");
        Rect {
            left: self.left,
            top: self.top,
            width,
            height,
        }
    }
}
