use core::ops::IndexMut;
use core::{marker::PhantomData, ptr::NonNull};

use super::geometry::{GRect, GSize};
use alloc::{boxed::Box, vec::Vec};
use gravel_sys::graphics::{bitmap as sys, color::GColor};

pub use sys::GBitmapFormat;

pub struct BitmapV2<Fmt, Mem> {
    inner: *mut sys::GBitmap,
    full_bounds: GRect,
    format: PhantomData<Fmt>,
    mem: PhantomData<Mem>,
}

impl<Fmt, Mem> BitmapV2<Fmt, Mem> {
    unsafe fn try_from_raw(inner: *mut sys::GBitmap) -> Option<Self> {
        (!inner.is_null()).then(|| Self {
            inner,
            full_bounds: unsafe { sys::gbitmap_get_bounds(inner) },
            format: PhantomData,
            mem: PhantomData,
        })
    }

    pub fn as_raw(&self) -> *mut sys::GBitmap {
        self.inner
    }

    pub fn format(&self) -> GBitmapFormat {
        unsafe { sys::gbitmap_get_format(self.inner) }
    }

    pub fn raw_data(&self) -> *mut u8 {
        unsafe { sys::gbitmap_get_data(self.inner) }
    }

    pub fn full_bounds(&self) -> GRect {
        self.full_bounds
    }

    pub fn bounds(&self) -> GRect {
        unsafe { sys::gbitmap_get_bounds(self.inner) }
    }

    pub fn set_bounds(&mut self, mut bounds: GRect) {
        bounds.clip(&self.full_bounds);
        unsafe { sys::gbitmap_set_bounds(self.inner, bounds) }
    }
}

impl BitmapV2<UnknownFormat, Owned> {
    pub fn from_png(png_data: &[u8]) -> Option<Self> {
        unsafe {
            let inner = sys::gbitmap_create_from_png_data(png_data.as_ptr(), png_data.len());
            Self::try_from_raw(inner)
        }
    }
}

impl<Mem> BitmapV2<UnknownFormat, Mem> {
    pub fn into_known_format(self) -> DynamicBitmap<Mem> {
        macro_rules! it {
            ($variant:ident) => {{
                let this = core::mem::ManuallyDrop::new(self);
                DynamicBitmap::$variant(BitmapV2 {
                    inner: this.inner,
                    full_bounds: this.full_bounds,
                    format: PhantomData,
                    mem: PhantomData,
                })
            }};
        }
        match self.format() {
            GBitmapFormat::BIT_1 => it!(Raw1Bit),
            GBitmapFormat::BIT_8 => it!(Raw8Bit),
            GBitmapFormat::CIRC_8 => it!(Round8Bit),
            GBitmapFormat::PAL_1 => it!(Pal1Bit),
            GBitmapFormat::PAL_2 => it!(Pal2Bit),
            GBitmapFormat::PAL_4 => it!(Pal4Bit),
            _ => unreachable!(),
        }
    }
}

pub enum DynamicBitmap<Mem> {
    Raw1Bit(BitmapV2<Raw1Bit, Mem>),
    Raw8Bit(BitmapV2<Raw8Bit, Mem>),
    Round8Bit(BitmapV2<Round8Bit, Mem>),
    Pal1Bit(BitmapV2<Pal1Bit, Mem>),
    Pal2Bit(BitmapV2<Pal2Bit, Mem>),
    Pal4Bit(BitmapV2<Pal4Bit, Mem>),
}

impl<Fmt: BitmapFormat> BitmapV2<Fmt, Owned> {
    pub fn create_blank(size: GSize) -> Option<Self> {
        unsafe {
            let inner = sys::gbitmap_create_blank(size, Fmt::FORMAT);
            Self::try_from_raw(inner)
        }
    }
}

impl<Fmt: PaletteFormat> BitmapV2<Fmt, Owned> {
    // Shared palettes would require... a third type parameter? to represent safely
    pub fn create_blank_with_palette(size: GSize, palette: Fmt::PaletteArray) -> Option<Self> {
        unsafe {
            let palette_ptr = Box::into_raw(Box::new(palette)).cast::<GColor>();
            let inner =
                sys::gbitmap_create_blank_with_palette(size, Fmt::FORMAT, palette_ptr, true);
            Self::try_from_raw(inner)
        }
    }

    pub fn palette(&self) -> &Fmt::PaletteArray {
        unsafe { &*sys::gbitmap_get_palette(self.inner).cast() }
    }

    pub fn palette_mut(&mut self) -> &mut Fmt::PaletteArray {
        unsafe { &mut *sys::gbitmap_get_palette(self.inner).cast() }
    }
}

impl<Fmt: RectangularFormat, Mem> BitmapV2<Fmt, Mem> {
    pub fn bytes_per_row(&self) -> u16 {
        unsafe { sys::gbitmap_get_bytes_per_row(self.inner) }
    }

    pub fn full_data_len(&self) -> usize {
        self.bytes_per_row() as usize * self.full_bounds.size.h as usize
    }

    pub unsafe fn raw_row(&self, y: u16) -> Option<NonNull<[u8]>> {
        let b = self.bounds();

        if y > u16::try_from(b.size.h).ok()? {
            return None;
        }

        let row_offset = (y as i16 + b.origin.y) as usize * self.bytes_per_row() as usize;

        unsafe {
            let start = self.raw_data().add(row_offset).add(b.origin.x as usize);
            let fat_ptr = core::ptr::slice_from_raw_parts_mut(start, b.size.w as usize);
            NonNull::new(fat_ptr)
        }
    }

    pub fn row(&self, y: u16) -> Option<Row<'_, Fmt>> {
        unsafe {
            let ptr = self.raw_row(y)?;
            Some(Row {
                data: ptr.as_ref(),
                format: PhantomData,
            })
        }
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> Option<Fmt::Pixel> {
        self.row(y).map(|r| r.get_pixel(x))
    }
}

impl<Fmt: RectangularFormat> BitmapV2<Fmt, Owned> {
    pub fn row_mut(&mut self, y: u16) -> Option<RowMut<'_, Fmt>> {
        unsafe {
            let mut ptr = self.raw_row(y)?;
            Some(RowMut {
                data: ptr.as_mut(),
                format: PhantomData,
            })
        }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, pixel: Fmt::Pixel) {
        if let Some(mut row) = self.row_mut(y) {
            row.set_pixel(x, pixel);
        }
    }
}

impl<Mem> BitmapV2<Round8Bit, Mem> {
    pub fn raw_row(&self, y: u16) -> Option<sys::GBitmapDataRowInfo> {
        let b = self.bounds();

        if y > u16::try_from(b.size.h).ok()? {
            return None;
        }

        let info = unsafe { sys::gbitmap_get_data_row_info(self.inner, y + b.origin.y as u16) };
        Some(info)
    }

    pub fn row(&self, y: u16) -> Option<(i16, Row<'_, Raw8Bit>)> {
        let info = self.raw_row(y)?;
        let data = unsafe {
            let ptr = info.data.add(info.min_x as usize);
            let len = (info.max_x - info.min_x) as usize;
            core::slice::from_raw_parts(ptr, len)
        };
        Some((
            info.min_x,
            Row {
                data,
                format: PhantomData,
            },
        ))
    }
}

impl BitmapV2<Round8Bit, Owned> {
    pub fn row_mut(&mut self, y: u16) -> Option<(i16, RowMut<'_, Raw8Bit>)> {
        let info = self.raw_row(y)?;
        let data = unsafe {
            let ptr = info.data.add(info.min_x as usize);
            let len = (info.max_x - info.min_x) as usize;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        Some((
            info.min_x,
            RowMut {
                data,
                format: PhantomData,
            },
        ))
    }
}

impl<Mem> BitmapV2<Raw1Bit, Mem> {
    pub fn create_palettized(&self) -> Option<BitmapV2<Pal1Bit, Owned>> {
        unsafe {
            let inner = sys::gbitmap_create_palettized_from_1bit(self.inner);
            BitmapV2::try_from_raw(inner)
        }
    }
}

pub struct Row<'a, Fmt> {
    data: &'a [u8],
    format: PhantomData<Fmt>,
}

pub struct RowMut<'a, Fmt> {
    data: &'a mut [u8],
    format: PhantomData<Fmt>,
}

impl<'a, Fmt: BitmapFormat> Row<'a, Fmt> {
    pub fn pixels(&self) -> impl Iterator<Item = Fmt::Pixel> + 'a {
        self.data.into_iter().copied().flat_map(Fmt::chunk)
    }

    pub fn len(&self) -> usize {
        core::mem::size_of::<Fmt::Chunk>() * self.data.len()
    }

    pub fn get_pixel(&self, x: u16) -> Fmt::Pixel {
        Fmt::get_pixel(self.data, x)
    }
}

impl<'a, Fmt: BitmapFormat> RowMut<'a, Fmt> {
    pub fn pixels(&self) -> impl Iterator<Item = Fmt::Pixel> + '_ {
        self.data.iter().copied().flat_map(Fmt::chunk)
    }

    pub fn len(&self) -> usize {
        core::mem::size_of::<Fmt::Chunk>() * self.data.len()
    }

    pub fn get_pixel(&self, x: u16) -> Fmt::Pixel {
        Fmt::get_pixel(self.data, x)
    }

    pub fn set_pixel(&mut self, x: u16, value: Fmt::Pixel) {
        Fmt::set_pixel(self.data, x, value)
    }
}

pub trait BitmapFormat: sealed::Sealed + 'static {
    type Pixel;
    type Chunk: 'static + IndexMut<usize, Output = Self::Pixel> + IntoIterator<Item = Self::Pixel>;
    const FORMAT: GBitmapFormat;

    fn get_pixel(row: &[u8], x: u16) -> Self::Pixel;
    fn set_pixel(row: &mut [u8], x: u16, value: Self::Pixel);
    fn chunk(byte: u8) -> Self::Chunk;
}

pub trait RectangularFormat: BitmapFormat {}
pub trait PaletteFormat: BitmapFormat<Pixel = u8> {
    type PaletteArray: AsMut<[GColor]>;
}

pub enum UnknownFormat {}

pub enum Raw1Bit {}
pub enum Raw8Bit {}
pub enum Pal1Bit {}
pub enum Pal2Bit {}
pub enum Pal4Bit {}
pub enum Round8Bit {}

impl BitmapFormat for Raw1Bit {
    type Pixel = bool;
    type Chunk = [bool; 8];
    const FORMAT: GBitmapFormat = GBitmapFormat::BIT_1;

    fn get_pixel(row: &[u8], x: u16) -> Self::Pixel {
        let byte = row[x as usize / 8];
        let mask = 0b1000_0000 >> (x & 7);
        byte & mask != 0
    }
    fn set_pixel(row: &mut [u8], x: u16, value: Self::Pixel) {
        let i = x as usize / 8;
        let byte = row[i];
        let mask = 0b1000_0000 >> (x & 7);
        row[i] = if value { byte | mask } else { byte & !mask };
    }
    fn chunk(byte: u8) -> Self::Chunk {
        core::array::from_fn::<_, 8, _>(|i| byte & (0x80 >> i) != 0)
    }
}
impl BitmapFormat for Raw8Bit {
    type Pixel = GColor;
    type Chunk = [GColor; 1];
    const FORMAT: GBitmapFormat = GBitmapFormat::BIT_8;
    fn get_pixel(row: &[u8], x: u16) -> Self::Pixel {
        let argb = row[x as usize];
        GColor { argb }
    }
    fn set_pixel(row: &mut [u8], x: u16, value: Self::Pixel) {
        row[x as usize] = value.argb;
    }
    fn chunk(byte: u8) -> Self::Chunk {
        [GColor { argb: byte }]
    }
}
impl BitmapFormat for Round8Bit {
    type Pixel = GColor;
    type Chunk = [GColor; 1];
    const FORMAT: GBitmapFormat = GBitmapFormat::CIRC_8;
    fn get_pixel(row: &[u8], x: u16) -> Self::Pixel {
        Raw8Bit::get_pixel(row, x)
    }
    fn set_pixel(row: &mut [u8], x: u16, value: Self::Pixel) {
        Raw8Bit::set_pixel(row, x, value)
    }
    fn chunk(byte: u8) -> Self::Chunk {
        [GColor { argb: byte }]
    }
}
impl BitmapFormat for Pal1Bit {
    type Pixel = u8;
    type Chunk = [u8; 8];
    const FORMAT: GBitmapFormat = GBitmapFormat::PAL_1;
    fn get_pixel(row: &[u8], x: u16) -> Self::Pixel {
        Raw1Bit::get_pixel(row, x) as u8
    }
    fn set_pixel(row: &mut [u8], x: u16, value: Self::Pixel) {
        Raw1Bit::set_pixel(row, x, value != 0)
    }
    fn chunk(byte: u8) -> Self::Chunk {
        Raw1Bit::chunk(byte).map(|b| b as u8)
    }
}
impl BitmapFormat for Pal2Bit {
    type Pixel = u8;
    type Chunk = [u8; 4];
    const FORMAT: GBitmapFormat = GBitmapFormat::PAL_2;
    fn get_pixel(row: &[u8], x: u16) -> Self::Pixel {
        let byte = row[x as usize / 4];
        let shift = (x as u32 & 3) * 2 + 2;
        byte.rotate_left(shift) & 3
    }
    fn set_pixel(row: &mut [u8], x: u16, value: Self::Pixel) {
        let i = x as usize / 4;
        let mut byte = row[i];
        let shift = (x as u32 & 3) * 2 + 2;
        byte = byte.rotate_left(shift);
        byte &= !3;
        byte |= value & 3;
        byte = byte.rotate_right(shift);
        row[i] = byte;
    }
    fn chunk(byte: u8) -> Self::Chunk {
        [byte >> 6, byte >> 4 & 3, byte >> 2 & 3, byte & 3]
    }
}
impl BitmapFormat for Pal4Bit {
    type Pixel = u8;
    type Chunk = [u8; 2];
    const FORMAT: GBitmapFormat = GBitmapFormat::PAL_4;
    fn get_pixel(row: &[u8], x: u16) -> Self::Pixel {
        let byte = row[x as usize >> 1];
        if x & 1 != 0 { byte & 0b1111 } else { byte >> 4 }
    }
    fn set_pixel(row: &mut [u8], x: u16, value: Self::Pixel) {
        let i = x as usize >> 1;
        let mut byte = row[i];
        if x & 1 == 0 {
            byte = byte.rotate_right(4);
        }
        byte &= !0b1111;
        byte |= value & 0b1111;
        if x & 1 == 0 {
            byte = byte.rotate_right(4);
        }
        row[i] = byte;
    }
    fn chunk(byte: u8) -> Self::Chunk {
        [byte >> 4, byte & 0b1111]
    }
}

impl RectangularFormat for Raw1Bit {}
impl RectangularFormat for Raw8Bit {}
impl RectangularFormat for Pal1Bit {}
impl RectangularFormat for Pal2Bit {}
impl RectangularFormat for Pal4Bit {}

impl PaletteFormat for Pal1Bit {
    type PaletteArray = [GColor; 2];
}
impl PaletteFormat for Pal2Bit {
    type PaletteArray = [GColor; 4];
}
impl PaletteFormat for Pal4Bit {
    type PaletteArray = [GColor; 16];
}

pub struct Shared<'a>(PhantomData<&'a [u8]>);
pub struct Owned(PhantomData<Vec<u8>>);

pub trait BitmapMemory {}
impl BitmapMemory for Shared<'_> {}
impl BitmapMemory for Owned {}

impl<Fmt, Mem> Drop for BitmapV2<Fmt, Mem> {
    fn drop(&mut self) {
        unsafe { sys::gbitmap_destroy(self.inner) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<'a> Sealed for super::Shared<'_> {}
    impl Sealed for super::Owned {}
    impl Sealed for super::UnknownFormat {}
    impl Sealed for super::Raw1Bit {}
    impl Sealed for super::Raw8Bit {}
    impl Sealed for super::Pal1Bit {}
    impl Sealed for super::Pal2Bit {}
    impl Sealed for super::Pal4Bit {}
    impl Sealed for super::Round8Bit {}
}

pub struct Bitmap {
    inner: *mut sys::GBitmap,
}

impl Bitmap {
    pub unsafe fn try_from_raw(inner: *mut sys::GBitmap) -> Option<Self> {
        (!inner.is_null()).then_some(Self { inner })
    }

    pub unsafe fn as_raw(&self) -> *mut sys::GBitmap {
        self.inner
    }

    pub fn from_png(data: &[u8]) -> Option<Self> {
        unsafe { Self::try_from_raw(sys::gbitmap_create_from_png_data(data.as_ptr(), data.len())) }
    }

    pub fn blank(size: GSize, format: GBitmapFormat) -> Option<Self> {
        unsafe { Self::try_from_raw(sys::gbitmap_create_blank(size, format)) }
    }

    // TODO: make a nice enum for the format
    pub fn format(&self) -> GBitmapFormat {
        unsafe { sys::gbitmap_get_format(self.inner) }
    }

    pub fn bounds(&self) -> GRect {
        unsafe { sys::gbitmap_get_bounds(self.inner) }
    }

    pub unsafe fn set_bounds(&mut self, bounds: GRect) {
        unsafe { sys::gbitmap_set_bounds(self.inner, bounds) }
    }

    pub unsafe fn get_buffer(&self) -> *mut u8 {
        unsafe { sys::gbitmap_get_data(self.inner) }
    }

    pub unsafe fn set_buffer(
        &mut self,
        new_buffer: *mut u8,
        format: GBitmapFormat,
        row_size_bytes: u16,
        free_on_destroy: bool,
    ) {
        unsafe {
            sys::gbitmap_set_data(
                self.inner,
                new_buffer,
                format,
                row_size_bytes,
                free_on_destroy,
            )
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { sys::gbitmap_destroy(self.inner) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bw_get_pixel() {
        let row = &[0x5C];
        assert_eq!(Raw1Bit::get_pixel(row, 0), false);
        assert_eq!(Raw1Bit::get_pixel(row, 1), true);
        assert_eq!(Raw1Bit::get_pixel(row, 2), false);
        assert_eq!(Raw1Bit::get_pixel(row, 3), true);
        assert_eq!(Raw1Bit::get_pixel(row, 4), true);
        assert_eq!(Raw1Bit::get_pixel(row, 5), true);
        assert_eq!(Raw1Bit::get_pixel(row, 6), false);
        assert_eq!(Raw1Bit::get_pixel(row, 7), false);
    }

    #[test]
    fn bw_set_pixel() {
        let mut row = [0];
        Raw1Bit::set_pixel(&mut row, 0, false);
        Raw1Bit::set_pixel(&mut row, 1, true);
        Raw1Bit::set_pixel(&mut row, 2, false);
        Raw1Bit::set_pixel(&mut row, 3, true);
        Raw1Bit::set_pixel(&mut row, 4, true);
        Raw1Bit::set_pixel(&mut row, 5, true);
        Raw1Bit::set_pixel(&mut row, 6, false);
        Raw1Bit::set_pixel(&mut row, 7, false);
    }

    #[test]
    fn pal2_get_pixel() {
        let row = &[0b00_01_10_11];
        assert_eq!(Pal2Bit::get_pixel(row, 0), 0b00);
        assert_eq!(Pal2Bit::get_pixel(row, 1), 0b01);
        assert_eq!(Pal2Bit::get_pixel(row, 2), 0b10);
        assert_eq!(Pal2Bit::get_pixel(row, 3), 0b11);

        assert_eq!(Pal2Bit::chunk(0b00_01_10_11), [0b00, 0b01, 0b10, 0b11]);
    }

    #[test]
    fn pal2_set_pixel() {
        let mut row = [0];
        Pal2Bit::set_pixel(&mut row, 0, 0b00);
        Pal2Bit::set_pixel(&mut row, 1, 0b01);
        Pal2Bit::set_pixel(&mut row, 2, 0b10);
        Pal2Bit::set_pixel(&mut row, 3, 0b11);
        assert_eq!(row, [0b00_01_10_11])
    }

    #[test]
    fn pal4_get_pixel() {
        let row = &[0b0000_0001, 0b0010_0100];
        assert_eq!(Pal4Bit::get_pixel(row, 0), 0b0000);
        assert_eq!(Pal4Bit::get_pixel(row, 1), 0b0001);
        assert_eq!(Pal4Bit::get_pixel(row, 2), 0b0010);
        assert_eq!(Pal4Bit::get_pixel(row, 3), 0b0100);
    }

    #[test]
    fn pal4_set_pixel() {
        let mut row = [0, 0];
        Pal4Bit::set_pixel(&mut row, 0, 0b0001);
        Pal4Bit::set_pixel(&mut row, 1, 0b0010);
        Pal4Bit::set_pixel(&mut row, 2, 0b0100);
        Pal4Bit::set_pixel(&mut row, 3, 0b1000);
        println!("{:08b} {:08b}", row[0], row[1]);
        assert_eq!(row, [0b0001_0010, 0b0100_1000]);
    }
}
