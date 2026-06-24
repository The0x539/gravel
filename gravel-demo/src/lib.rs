#![no_std]

extern crate alloc;

use gravel::foundation::platform;
use gravel::foundation::watch_info;
use gravel::graphics::Bitmap;
use gravel::graphics::color::GColor;
use gravel::graphics::geometry::{GPoint, GRect, GSize};
use gravel::prelude::*;
use gravel::ui::layer::BitmapLayer;
use gravel::ui::window::Window;
use gravel::ui::window_stack;

const MRGREEN: &[u8] = include_bytes!("./mrgreen.png");

#[unsafe(no_mangle)]
pub extern "C" fn main() -> isize {
    let window = Window::new();

    unsafe {
        gravel::log!(
            Info,
            c"model/color: %d/%d\n",
            watch_info::model(),
            watch_info::color(),
        );
    }

    window.set_background_color(GColor::PICTON_BLUE);
    window_stack::push(&window, true);

    let unscaled = Bitmap::from_png(MRGREEN).unwrap();
    // assume BIT_8 format in this proof of concept
    let dst_bounds = unscaled.bounds();
    let scaled = Bitmap::blank(platform::DISPLAY_SIZE, unscaled.format()).unwrap();
    unsafe {
        let src_bounds = scaled.bounds();
        let src_buf = unscaled.get_buffer();
        let dst_buf = scaled.get_buffer();

        for dst_y in 0..src_bounds.size.h {
            let src_y = dst_y * dst_bounds.size.h / src_bounds.size.h;
            for dst_x in 0..src_bounds.size.w {
                let src_x = dst_x * dst_bounds.size.w / src_bounds.size.w;
                let dst_i = from_coords(GPoint::new(dst_x, dst_y), src_bounds.size);
                let src_i = from_coords(GPoint::new(src_x, src_y), dst_bounds.size);

                let pixel = *src_buf.add(src_i);
                *dst_buf.add(dst_i) = pixel;
            }
        }
    }

    let size = platform::DISPLAY_SIZE;

    let mut frame = GRect::default();
    frame.size = size;
    let mut layer = BitmapLayer::new(frame).unwrap();
    layer.set_bitmap(&scaled);
    let root_layer = window.root_layer();
    root_layer.add_child(&layer.get_layer());

    #[cfg(device_feature = "touch")]
    gravel::foundation::event::touch::subscribe(move |event| unsafe {
        let x_range = 0..size.x;
        let y_range = 0..size.y;

        for dy in -2..=2 {
            if !y_range.contains(&(event.y + dy)) {
                continue;
            }

            for dx in -2..=2 {
                if !x_range.contains(&(event.x + dx)) {
                    continue;
                }

                let i = from_coords(GPoint::new(event.x + dx, event.y + dy), size);
                let pixel = scaled.get_buffer().add(i);
                *pixel = !*pixel;
            }
        }

        layer.get_layer().mark_dirty();
    });

    gravel::foundation::app::event_loop();

    0
}

fn from_coords(point: GPoint, dims: GSize) -> usize {
    point.y as usize * dims.w as usize + point.x as usize
}
