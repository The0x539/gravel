use core::ffi::{c_int, c_void};

unsafe extern "C" {
    pub fn psleep(millis: c_int);
    pub fn app_timer_register(
        timeout_ms: u32,
        callback: AppTimerCallback,
        callback_data: *mut c_void,
    ) -> *mut AppTimer;
    pub fn app_timer_reschedule(timer_handle: *mut AppTimer, new_timeout_ms: u32) -> bool;
    pub fn app_timer_cancel(timer_handle: *mut AppTimer);
}

#[repr(C)]
pub struct AppTimer(crate::Opaque);

callbacks! {
    pub fn AppTimerCallback(data: *mut c_void);
}
