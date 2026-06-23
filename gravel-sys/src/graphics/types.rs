use crate::util::Opaque;

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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GColor8 {
    pub argb: u8,
}

impl GColor8 {
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        let a___ = (a & 3) << 6;
        let _r__ = (r & 3) << 4;
        let __g_ = (g & 3) << 2;
        let ___b = (b & 3) << 0;
        let argb = a___ | _r__ | __g_ | ___b;
        Self { argb }
    }
}

#[rustfmt::skip]
impl GColor8 {
    pub const fn a(&self) -> u8 { (self.argb >> 6) & 3 }
    pub const fn r(&self) -> u8 { (self.argb >> 4) & 3 }
    pub const fn g(&self) -> u8 { (self.argb >> 2) & 3 }
    pub const fn b(&self) -> u8 { (self.argb >> 0) & 3 }
}

macro_rules! named_colors {
    (
        $($r:literal $g:literal $b:literal -> $name:ident)*
    ) => {
        impl GColor8 {
            $(pub const $name: Self = Self::from_argb(3, $r, $g, $b);)*
        }
    }
}

named_colors! {
     0 0 0 -> BLACK
     0 0 1 -> OXFORD_BLUE
     0 0 2 -> DUKE_BLUE
     0 0 3 -> BLUE
     0 1 0 -> DARK_GREEN
     0 1 1 -> MIDNIGHT_GREEN
     0 1 2 -> COBALT_BLUE
     0 1 3 -> BLUE_MOON
     0 2 0 -> ISLAMIC_GREEN
     0 2 1 -> JAEGER_GREEN
     0 2 2 -> TIFFANY_BLUE
     0 2 3 -> VIVID_CERULEAN
     0 3 0 -> GREEN
     0 3 1 -> MALACHITE
     0 3 2 -> MEDIUM_SPRING_GREEN
     0 3 3 -> CYAN
     1 0 0 -> BULGARIAN_ROSE
     1 0 1 -> IMPERIAL_PURPLE
     1 0 2 -> INDIGO
     1 0 3 -> ELECTRIC_ULTRAMARINE
     1 1 0 -> ARMY_GREEN
     1 1 1 -> DARK_GRAY
     1 1 2 -> LIBERTY
     1 1 3 -> VERY_LIGHT_BLUE
     1 2 0 -> KELLY_GREEN
     1 2 1 -> MAY_GREEN
     1 2 2 -> CADET_BLUE
     1 2 3 -> PICTON_BLUE
     1 3 0 -> BRIGHT_GREEN
     1 3 1 -> SCREAMIN_GREEN
     1 3 2 -> MEDIUM_AQUAMARINE
     1 3 3 -> ELECTRIC_BLUE
     2 0 0 -> DARK_CANDY_APPLE_RED
     2 0 1 -> JAZZBERRY_JAM
     2 0 2 -> PURPLE
     2 0 3 -> VIVID_VIOLET
     2 1 0 -> WINDSOR_TAN
     2 1 1 -> ROSE_VALE
     2 1 2 -> PURPUREUS
     2 1 3 -> LAVENDER_INDIGO
     2 2 0 -> LIMERICK
     2 2 1 -> BRASS
     2 2 2 -> LIGHT_GRAY
     2 2 3 -> BABY_BLUE_EYES
     2 3 0 -> SPRING_BUD
     2 3 1 -> INCHWORM
     2 3 2 -> MINT_GREEN
     2 3 3 -> CELESTE
     3 0 0 -> RED
     3 0 1 -> FOLLY
     3 0 2 -> FASHION_MAGENTA
     3 0 3 -> MAGENTA
     3 1 0 -> ORANGE
     3 1 1 -> SUNSET_ORANGE
     3 1 2 -> BRILLIANT_ROSE
     3 1 3 -> SHOCKING_PINK
     3 2 0 -> CHROME_YELLOW
     3 2 1 -> RAJAH
     3 2 2 -> MELON
     3 2 3 -> RICH_BRILLIANT_LAVENDER
     3 3 0 -> YELLOW
     3 3 1 -> ICTERINE
     3 3 2 -> PASTEL_YELLOW
     3 3 3 -> WHITE
}

pub type GColor = GColor8;

#[repr(C)]
pub struct GContext(Opaque);

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GCompOp {
    Assign,
    AssignInverted,
    Or,
    And,
    Clear,
    Set,
}
