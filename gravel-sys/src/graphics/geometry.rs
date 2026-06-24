#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GPoint {
    pub x: i16,
    pub y: i16,
}

impl GPoint {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GSize {
    pub w: i16,
    pub h: i16,
}

impl GSize {
    pub const fn new(w: i16, h: i16) -> Self {
        Self { w, h }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GRect {
    pub origin: GPoint,
    pub size: GSize,
}

macro_rules! ffi_methods {
    ($(
        $(#[$attr:meta])*
        $vis:vis fn $method:ident
        ( $($arg:ident : $aty:ty),* $(,)? )
        $( -> $ret:ty )? = $ffi:path;
    )*) => {$(
        $(#[$attr])*
        $vis fn $method($($arg: $aty),*) $(-> $ret)? {
            unsafe { $ffi($($arg),*) }
        }
    )*};
}

impl GRect {
    pub const fn new(origin: GPoint, size: GSize) -> Self {
        Self { origin, size }
    }

    pub const fn from_xywh(x: i16, y: i16, w: i16, h: i16) -> Self {
        Self::new(GPoint { x, y }, GSize { w, h })
    }

    pub const fn from_ltrb(top: i16, right: i16, bottom: i16, left: i16) -> Self {
        Self::from_xywh(left, top, right - left, bottom - top)
    }

    pub const fn is_empty(&self) -> bool {
        self.size.w == 0 && self.size.h == 0
    }

    pub const fn standardize(&mut self) {
        if self.size.w < 0 {
            self.origin.x += self.size.w;
            self.size.w = -self.size.w;
        }
        if self.size.h < 0 {
            self.origin.y += self.size.h;
            self.size.h = -self.size.h;
        }
    }

    #[must_use]
    pub const fn standardized(mut self) -> Self {
        self.standardize();
        self
    }

    pub const fn center_point(&self) -> GPoint {
        GPoint {
            x: self.origin.x + self.size.w / 2,
            y: self.origin.y + self.size.h / 2,
        }
    }

    pub const fn max_x(&self) -> i16 {
        self.origin.x + self.size.w
    }

    pub const fn max_y(&self) -> i16 {
        self.origin.y + self.size.h
    }

    pub fn longest_side(&self) -> i16 {
        i16::max(self.size.w.abs(), self.size.h.abs())
    }

    pub fn shortest_side(&self) -> i16 {
        i16::min(self.size.w.abs(), self.size.h.abs())
    }

    ffi_methods! {
        #[doc(alias = "intersect")]
        pub fn clip(self: &mut Self, clipper: &Self) = grect_clip;
        pub fn union(r1: &Self, r2: &Self) -> Self = grect_union;
        pub fn contains_point(self: &Self, point: &GPoint) -> bool = grect_contains_point;
        #[must_use]
        pub fn crop(self: Self, crop_size_px: i32) -> Self = grect_crop;
        #[must_use]
        pub fn inset(self: Self, insets: GEdgeInsets) -> Self = grect_inset;
        pub fn align(self: &mut Self, inside_rect: &GRect, alignment: GAlign, clip: bool) = grect_align;
    }
}

unsafe extern "C" {
    fn grect_clip(rect_to_clip: *mut GRect, rect_clipper: *const GRect);
    fn grect_union(r1: *const GRect, r2: *const GRect) -> GRect;
    fn grect_contains_point(rect: *const GRect, point: *const GPoint) -> bool;
    fn grect_crop(rect: GRect, crop_size_px: i32) -> GRect;
    fn grect_inset(rect: GRect, insets: GEdgeInsets) -> GRect;
    fn grect_align(rect: *mut GRect, inside_rect: *const GRect, alignment: GAlign, clip: bool);
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct GEdgeInsets {
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
    pub left: i16,
}

impl GEdgeInsets {
    pub const fn t_r_b_l(top: i16, right: i16, bottom: i16, left: i16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn t_rl_b(t: i16, rl: i16, b: i16) -> Self {
        Self::t_r_b_l(t, rl, b, rl)
    }

    pub const fn tb_rl(tb: i16, rl: i16) -> Self {
        Self::t_r_b_l(tb, rl, tb, rl)
    }

    pub const fn uniform(v: i16) -> Self {
        Self::t_r_b_l(v, v, v, v)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GAlign {
    Center,
    TopLeft,
    TopRight,
    Top,
    Left,
    Bottom,
    Right,
    BottomRight,
    BottomLeft,
}
