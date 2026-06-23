#![no_std]

extern crate alloc;

#[macro_use]
mod util;

mod allocator;
pub use allocator::PebbleAllocator;

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
    pub mod event {
        pub mod accelerometer;
        pub mod app_focus;
        pub mod battery_state;
        pub mod compass;
        pub mod connection;
        // pub mod health;
        pub mod tick_timer;
        #[cfg(device_feature = "touch")]
        pub mod touch;
    }
    pub mod exit_reason;
    pub mod i18n;
    pub mod launch_reason;
    pub mod logging;
    pub mod math;
    pub mod memory_management;
    pub mod platform;
    // pub mod resources;
    // pub mod storage;
    // pub mod timer;
    // pub mod wakeup;
    // pub mod wall_time;
    // pub mod watch_info;
}

pub mod graphics;

pub mod ui {
    // pub mod animation;
    pub mod clicks;
    pub mod layer;
    // pub mod light;
    // pub mod preferences;
    // pub mod speaker;
    // pub mod unobstructed_area;
    // pub mod vibes;
    pub mod window;
    pub mod window_stack;
}

pub mod prelude {
    pub use crate::ui::layer::LayerHandle;
    pub use crate::ui::window::WindowHandle;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
