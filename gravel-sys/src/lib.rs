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
}

pub mod std_c;
