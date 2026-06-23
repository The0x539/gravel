use core::ffi::c_void;

unsafe extern "C" {
    pub fn resource_get_handle(resource_id: u32) -> ResHandle;
    pub fn resource_size(h: ResHandle) -> usize;
    pub fn resource_load(h: ResHandle, buffer: *mut u8, max_length: usize) -> usize;
    pub fn resource_load_byte_range(
        h: ResHandle,
        start_offset: u32,
        buffer: *mut u8,
        num_bytes: usize,
    ) -> usize;
}

pub type ResHandle = *mut c_void;
