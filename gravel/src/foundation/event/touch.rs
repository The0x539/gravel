use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::ffi::c_void;
use gravel_sys::foundation::event::touch as sys;
pub use sys::TouchEvent;

pub trait TouchServiceHandler: FnMut(&TouchEvent) + 'static {}
impl<F: FnMut(&TouchEvent) + 'static> TouchServiceHandler for F {}

extern "C" fn sys_handler(event: *const TouchEvent, _: *mut c_void) {
    if let Some(f) = GLOBAL_HANDLER.get_ref() {
        unsafe { f(&*event) }
    }
}

pub fn subscribe<H: TouchServiceHandler>(handler: H) {
    if GLOBAL_HANDLER.replace(Box::new(handler)).is_none() {
        unsafe { sys::touch_service_subscribe(sys_handler, core::ptr::null_mut()) }
    }
}

pub fn unsubscribe() {
    if GLOBAL_HANDLER.take().is_some() {
        unsafe { sys::touch_service_unsubscribe() }
    }
}

pub fn is_enabled() -> bool {
    unsafe { sys::touch_service_is_enabled() }
}

// It's kinda nuts that I needed to implement this using a global variable.
// This is necessary in order to run the destructor of a previous subscription.
// As a side effect of this, the context parameter isn't used for this module,
// because we can store everything we need inside the global variable.
static GLOBAL_HANDLER: GlobalCell<Box<dyn TouchServiceHandler>> = GlobalCell::new();

struct GlobalCell<T>(UnsafeCell<Option<T>>);

/// SAFETY: This platform does not have threads.
unsafe impl<T> Sync for GlobalCell<T> {}

impl<T> GlobalCell<T> {
    const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    fn get_ref(&self) -> &mut Option<T> {
        unsafe { &mut *self.0.get() }
    }

    fn take(&self) -> Option<T> {
        self.get_ref().take()
    }

    fn replace(&self, value: T) -> Option<T> {
        self.get_ref().replace(value)
    }
}
