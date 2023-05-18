use ::safer_ffi::prelude::*;


#[ffi_export]
pub fn hello_world() {
    println!("Hello, world!");
}

#[derive_ReprC]
pub use libremarkable::cgmath::Point2;

use libremarkable::framebuffer::common;
use libremarkable::framebuffer::FramebufferDraw;

#[derive_ReprC]
pub use libremarkable::framebuffer::core::Framebuffer;

#[derive_ReprC]
pub use libremarkable::framebuffer::common::mxcfb_rect;

#[ffi_export]
pub fn framebuffer_new() -> &mut Framebuffer {
    Framebuffer::new()
}

#[derive_ReprC]
#[repr(C)]
pub enum Color {
    Black,
    Gray(u8)
}

#[ffi_export]
pub fn framebuffer_draw_line(framebuffer: &mut Framebuffer, start: Point2<i32>, end: Point2<i32>, width: u32, v: Color) -> mxcfb_rect {

    let c: common::color = match v {
        Color::Black => common::color::BLACK,
        Color::Gray(v) => common::color::GRAY(v)
    };

    framebuffer.draw_line(start, end, width, c)
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("libremarkable_c.h")
        .generate()
}
