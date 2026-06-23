use core::mem::MaybeUninit;

use gravel_sys::foundation::resources as sys;

// TODO: The numeric resource IDs emitted as a C header aren't currently visible to Rust code.
// Not yet sure how to best resolve this.

#[derive(Debug)]
pub struct Resource {
    inner: sys::ResHandle,
}

impl Resource {
    pub fn get(id: u32) -> Option<Self> {
        let inner = unsafe { sys::resource_get_handle(id) };
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    pub fn size(&self) -> usize {
        unsafe { sys::resource_size(self.inner) }
    }

    pub fn load(&self, buffer: &mut [u8]) -> usize {
        let (ptr, len) = (buffer.as_mut_ptr(), buffer.len());
        unsafe { sys::resource_load(self.inner, ptr, len) }
    }

    pub fn load_buf(&self, buffer: &mut [MaybeUninit<u8>]) -> usize {
        let (ptr, len) = (buffer.as_mut_ptr().cast(), buffer.len());
        unsafe { sys::resource_load(self.inner, ptr, len) }
    }

    pub fn load_byte_range(&self, start_offset: u32, buffer: &mut [u8]) -> usize {
        let (ptr, len) = (buffer.as_mut_ptr(), buffer.len());
        unsafe { sys::resource_load_byte_range(self.inner, start_offset, ptr, len) }
    }

    pub fn load_byte_range_buf(&self, start_offset: u32, buffer: &mut [MaybeUninit<u8>]) -> usize {
        let (ptr, len) = (buffer.as_mut_ptr().cast(), buffer.len());
        unsafe { sys::resource_load_byte_range(self.inner, start_offset, ptr, len) }
    }
}
