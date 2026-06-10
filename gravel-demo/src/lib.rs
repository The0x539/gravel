#![no_std]

use gravel::graphics::types::*;
use gravel::prelude::*;
use gravel::ui::window::Window;
use gravel::ui::window_stack;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> isize {
    let window = Window::new();
    window.set_background_color(GColor::from_argb(3, 1, 2, 3));
    window_stack::push(&window, true);
    gravel::foundation::app::event_loop();
    0
}
