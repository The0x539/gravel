#![no_std]

use gravel::foundation::platform;
use gravel::graphics::Bitmap;
use gravel::graphics::types::*;
use gravel::prelude::*;
use gravel::ui::layer::BitmapLayer;
use gravel::ui::window::Window;
use gravel::ui::window_stack;

use gravel::log;

const MRGREEN: &[u8] = include_bytes!("./mrgreen.png");

#[unsafe(no_mangle)]
pub extern "C" fn main() -> isize {
    unsafe {
        log!(
            Info,
            c"foo %d %d %d\n",
            platform::PlatformType::CURRENT,
            platform::DISPLAY_SIZE.x as i32,
            platform::DISPLAY_SIZE.y as i32,
        );
    }

    let window = Window::new();

    window.set_background_color(GColor::from_argb(3, 1, 2, 3));
    window_stack::push(&window, true);

    let bitmap = Bitmap::from_png(MRGREEN).unwrap();

    let mut frame = GRect::default();
    frame.size = platform::DISPLAY_SIZE;
    let mut layer = BitmapLayer::new(frame).unwrap();
    layer.set_bitmap(&bitmap);
    window.root_layer().add_child(&layer.get_layer());

    gravel::foundation::app::event_loop();

    0
}
