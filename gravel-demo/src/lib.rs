#![no_std]

#[unsafe(no_mangle)]
pub extern "C" fn main() -> isize {
    gravel::foundation::app::event_loop();
    0
}
