#![no_std]

extern crate alloc;

#[macro_use]
mod util;

mod allocator;

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
        pub mod touch;
    }
}

pub mod graphics {
    pub mod types {
        pub use gravel_sys::graphics::types::*;
    }
}

pub mod ui {
    pub mod clicks;
    pub mod layer;
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
