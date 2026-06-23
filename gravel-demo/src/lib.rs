#![no_std]

extern crate alloc;

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

    let size = GSize::new(
        platform::DISPLAY_SIZE.x * 2 / 3,
        platform::DISPLAY_SIZE.y * 2 / 3,
    );

    window.set_background_color(GColor::from_argb(3, 1, 2, 3));
    window_stack::push(&window, true);

    let unscaled = Bitmap::from_png(MRGREEN).unwrap();
    // assume BIT_8 format in this proof of concept
    let dst_bounds = unscaled.bounds();
    let scaled = Bitmap::blank(size, unscaled.format()).unwrap();
    unsafe {
        let src_bounds = scaled.bounds();
        let src_buf = unscaled.get_buffer();
        let dst_buf = scaled.get_buffer();

        for dst_y in 0..src_bounds.size.y {
            let src_y = dst_y * dst_bounds.size.y / src_bounds.size.y;
            for dst_x in 0..src_bounds.size.x {
                let src_x = dst_x * dst_bounds.size.x / src_bounds.size.x;
                let dst_i = from_coords(GPoint::new(dst_x, dst_y), src_bounds.size);
                let src_i = from_coords(GPoint::new(src_x, src_y), dst_bounds.size);

                let pixel = *src_buf.add(src_i);
                *dst_buf.add(dst_i) = pixel;
            }
        }
    }

    let mut frame = GRect::default();
    frame.size = platform::DISPLAY_SIZE;
    let mut layer = BitmapLayer::new(frame).unwrap();
    layer.set_bitmap(&scaled);
    window.root_layer().add_child(&layer.get_layer());

    gravel::foundation::app::event_loop();

    0
}

fn from_coords(point: GPoint, dims: GSize) -> usize {
    point.y as usize * dims.x as usize + point.x as usize
}
