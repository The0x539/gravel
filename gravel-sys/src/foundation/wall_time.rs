use core::ffi::{c_char, c_int};

use crate::std_c::time::time_t;

unsafe extern "C" {
    pub fn clock_copy_time_string(buffer: *mut c_char, size: u8);
    pub fn clock_is_24h_style() -> bool;
    pub fn clock_to_timestamp(day: WeekDay, hour: c_int, minute: c_int) -> time_t;
    pub fn clock_is_timezone_set() -> bool;
    pub fn clock_get_timezone(timezone: *mut c_char, buffer_size: usize);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeekDay {
    Today,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
