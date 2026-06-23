use alloc::boxed::Box;
use core::ffi::{c_int, c_void};

use gravel_sys::foundation::timer as sys;

#[derive(Debug)]
pub struct AppTimer {
    inner: *mut sys::AppTimer,
}

pub fn psleep(millis: c_int) {
    unsafe { sys::psleep(millis) }
}

impl AppTimer {
    pub fn register<F: AppTimerCallback>(timeout: impl Timeout, callback: F) -> Self {
        let boxed: Box<F> = Box::new(callback);
        let ptr: *mut F = Box::into_raw(boxed);
        let void_ptr: *mut c_void = ptr.cast();
        let inner = unsafe { sys::app_timer_register(timeout.into_ms(), F::handle_sys, void_ptr) };
        Self { inner }
    }

    pub fn reschedule(&self, new_timeout: impl Timeout) -> Result<(), AlreadyElapsed> {
        let ret = unsafe { sys::app_timer_reschedule(self.inner, new_timeout.into_ms()) };
        if ret { Ok(()) } else { Err(AlreadyElapsed) }
    }

    pub fn cancel(self) {
        unsafe { sys::app_timer_cancel(self.inner) }
    }
}

pub trait AppTimerCallback: FnOnce() + Sized {
    unsafe extern "C" fn handle_sys(data: *mut c_void) {
        let ptr: *mut Self = data.cast();
        let boxed: Box<Self> = unsafe { Box::from_raw(ptr) };
        boxed()
    }
}

impl<F: FnOnce() + Sized> AppTimerCallback for F {}

#[derive(Debug)]
pub struct AlreadyElapsed;

impl core::fmt::Display for AlreadyElapsed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Timer has already elapsed.")
    }
}

impl core::error::Error for AlreadyElapsed {}

pub trait Timeout {
    fn into_ms(self) -> u32;
}

impl Timeout for u32 {
    fn into_ms(self) -> u32 {
        self
    }
}

// Instant comes from std, unfortunately. At some point I'll reimplement it.

impl Timeout for core::time::Duration {
    fn into_ms(self) -> u32 {
        self.as_millis().try_into().unwrap_or(u32::MAX)
    }
}
