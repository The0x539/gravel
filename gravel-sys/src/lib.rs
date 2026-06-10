#![no_std]

#[macro_use]
mod util;
use util::Opaque;

pub mod foundation {
    pub mod alloy;
    pub mod app;
    pub mod data_logging;
    pub mod data_structures;
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
    pub mod types;
}

pub mod ui {
    pub mod clicks;
    pub mod layer;
    pub mod window;
    pub mod window_stack;
}

pub mod std_c;
