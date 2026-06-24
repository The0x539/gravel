#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct GColor8 {
    pub argb: u8,
}

pub type GColor = GColor8;

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
    ($($r:literal $g:literal $b:literal -> $name:ident)*) => {
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
