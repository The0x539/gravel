#![no_std]

extern crate alloc;

use gravel::foundation::platform;
use gravel::graphics::bitmap::Raw8Bit;
use gravel::graphics::bitmap::{BitmapV2, DynamicBitmap};
use gravel::graphics::color::GColor;
use gravel::graphics::geometry::GRect;
use gravel::prelude::*;
use gravel::ui::layer::BitmapLayer;
use gravel::ui::window::Window;
use gravel::ui::window_stack;

const MRGREEN: &[u8] = include_bytes!("./mrgreen.png");

#[unsafe(no_mangle)]
pub extern "C" fn main() -> isize {
    let window = Window::new();

    window.set_background_color(GColor::PICTON_BLUE);
    window_stack::push(&window, true);

    let unscaled = match BitmapV2::from_png(MRGREEN).unwrap().into_known_format() {
        DynamicBitmap::Raw8Bit(b) => b,
        _ => panic!(),
    };
    let src_bounds = unscaled.bounds();

    let mut scaled = BitmapV2::<Raw8Bit, _>::create_blank(platform::DISPLAY_SIZE).unwrap();

    let dst_bounds = GRect {
        size: platform::DISPLAY_SIZE,
        ..Default::default()
    };
    for dst_y in 0..dst_bounds.size.h {
        let src_y = dst_y as u32 * src_bounds.size.h as u32 / dst_bounds.size.h as u32;

        let mut dst_row = scaled.row_mut(dst_y as u16).unwrap();
        let src_row = unscaled.row(src_y as u16).unwrap();

        for dst_x in 0..dst_bounds.size.w {
            let src_x = dst_x as u32 * src_bounds.size.w as u32 / dst_bounds.size.w as u32;
            let pixel = src_row.get_pixel(src_x as u16);
            dst_row.set_pixel(dst_x as u16, pixel);
        }
    }

    // let mut bounds = scaled.bounds();
    // bounds.size.h /= 2;
    // bounds.origin.y += bounds.size.h / 2;
    // scaled.set_bounds(bounds);

    let size = platform::DISPLAY_SIZE;

    let mut frame = GRect::default();
    frame.size = size;
    let mut layer = BitmapLayer::new(frame).unwrap();
    layer.set_bitmap_v2(&scaled);
    let root_layer = window.root_layer();
    root_layer.add_child(&layer.get_layer());

    #[cfg(device_feature = "touch")]
    gravel::foundation::event::touch::subscribe(move |event| {
        let x_range = 0..size.w;
        let y_range = 0..size.h;

        for dy in -2..=2 {
            if !y_range.contains(&(event.y + dy)) {
                continue;
            }

            for dx in -2..=2 {
                if !x_range.contains(&(event.x + dx)) {
                    continue;
                }

                let x = (event.x + dx) as u16;
                let y = (event.y + dy) as u16;

                let Some(mut pixel) = scaled.get_pixel(x, y) else {
                    continue;
                };
                pixel.argb = !pixel.argb;
                scaled.set_pixel(x, y, pixel);
            }
        }

        layer.get_layer().mark_dirty();
    });

    gravel::foundation::app::event_loop();

    0
}
