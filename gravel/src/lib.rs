#![no_std]

extern crate alloc;

#[macro_use]
mod util;
use util::*;

pub use gravel_sys;

pub mod foundation {
    pub mod app {
        pub fn event_loop() {
            unsafe { gravel_sys::foundation::app::app_event_loop() }
        }

        pub mod comm;
        pub mod glance;
        pub mod message;
        pub mod sync;
        pub mod worker;
    }

    pub mod data_logging;
    pub mod dictation;
    pub mod dictionary;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
