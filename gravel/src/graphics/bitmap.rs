use super::geometry::{GRect, GSize};
use gravel_sys::graphics::bitmap as sys;

pub use sys::GBitmapFormat;

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

    /*
    pub unsafe fn row(&self, y: u16) -> BitmapRow<'_> {
        BitmapRow {
            inner: unsafe { sys::gbitmap_get_data_row_info(self.inner, y) },
            borrow: PhantomData,
        }
    }

    pub unsafe fn row_mut(&mut self, y: u16) -> BitmapRowMut<'_> {
        BitmapRowMut {
            inner: unsafe { sys::gbitmap_get_data_row_info(self.inner, y) },
            borrow: PhantomData,
        }
    }
    */

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

/*
pub struct BitmapRow<'a> {
    inner: sys::GBitmapDataRowInfo,
    borrow: PhantomData<&'a [u8]>,
}

impl BitmapRow<'_> {
    pub fn range(&self) -> RangeInclusive<i16> {
        self.inner.min_x..=self.inner.max_x
    }

    // I'm probably getting something very wrong here.
    pub unsafe fn as_slice(&self, format: GBitmapFormat) -> &[u8] {
        let (i, j) = match format {
            GBitmapFormat::BIT_1 | GBitmapFormat::PAL_1 => {
                (self.inner.min_x / 8, self.inner.max_x / 8)
            }
            GBitmapFormat::PAL_2 => (self.inner.min_x / 4, self.inner.max_x / 4),
            GBitmapFormat::PAL_4 => (self.inner.min_x / 2, self.inner.max_x / 2),
            _ => (self.inner.min_x, self.inner.max_x),
        };

        unsafe {
            let ptr = self.inner.data.cast_const();
            core::slice::from_raw_parts(ptr.add(i as usize), (j - i) as usize + 1)
        }
    }
}

pub struct BitmapRowMut<'a> {
    inner: sys::GBitmapDataRowInfo,
    borrow: PhantomData<&'a mut [u8]>,
}

impl BitmapRowMut<'_> {
    pub fn range(&self) -> RangeInclusive<i16> {
        self.inner.min_x..=self.inner.max_x
    }

    pub unsafe fn as_slice(&self, format: GBitmapFormat) -> &[u8] {
        let (i, j) = match format {
            GBitmapFormat::BIT_1 | GBitmapFormat::PAL_1 => {
                (self.inner.min_x / 8, self.inner.max_x / 8)
            }
            GBitmapFormat::PAL_2 => (self.inner.min_x / 4, self.inner.max_x / 4),
            GBitmapFormat::PAL_4 => (self.inner.min_x / 2, self.inner.max_x / 2),
            _ => (self.inner.min_x, self.inner.max_x),
        };

        unsafe {
            let ptr = self.inner.data;
            core::slice::from_raw_parts(ptr.add(i as usize).cast_const(), (j - i) as usize + 1)
        }
    }

    pub unsafe fn as_mut_slice(&mut self, format: GBitmapFormat) -> &mut [u8] {
        let (i, j) = match format {
            GBitmapFormat::BIT_1 | GBitmapFormat::PAL_1 => {
                (self.inner.min_x / 8, self.inner.max_x / 8)
            }
            GBitmapFormat::PAL_2 => (self.inner.min_x / 4, self.inner.max_x / 4),
            GBitmapFormat::PAL_4 => (self.inner.min_x / 2, self.inner.max_x / 2),
            _ => (self.inner.min_x, self.inner.max_x),
        };

        unsafe {
            let ptr = self.inner.data;
            core::slice::from_raw_parts_mut(ptr.add(i as usize), (j - i) as usize + 1)
        }
    }
}
*/
