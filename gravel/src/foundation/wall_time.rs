use core::ffi::{CStr, c_char, c_int};
use gravel_sys::foundation::wall_time as sys;
use gravel_sys::std_c::time::time_t;

pub use sys::WeekDay;

pub fn copy_time_string(buffer: &mut [c_char]) {
    unsafe {
        sys::clock_copy_time_string(
            buffer.as_mut_ptr(),
            buffer.len().try_into().unwrap_or(u8::MAX),
        )
    }
}

pub fn is_24h_style() -> bool {
    unsafe { sys::clock_is_24h_style() }
}

pub fn to_timestamp(day: WeekDay, hour: c_int, minute: c_int) -> time_t {
    unsafe { sys::clock_to_timestamp(day, hour, minute) }
}

pub fn is_timezone_set() -> bool {
    unsafe { sys::clock_is_timezone_set() }
}

pub fn get_timezone(buffer: &mut [u8; 32]) -> &CStr {
    unsafe { sys::clock_get_timezone(buffer.as_mut_ptr().cast(), buffer.len()) };
    CStr::from_bytes_until_nul(buffer).unwrap_or(c"")
}
