#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello, world!");
}

pub use libremarkable::cgmath::Point2;
use libremarkable::framebuffer::common;
use libremarkable::framebuffer::FramebufferDraw;

pub use libremarkable::framebuffer::core::Framebuffer;
pub use libremarkable::framebuffer::common::mxcfb_rect;

#[no_mangle]
pub extern "C" fn framebuffer_new() -> *mut Framebuffer {
    &mut Framebuffer::new() as *mut Framebuffer
}

#[repr(C)]
pub enum Color {
    Black,
    Gray(u8)
}

#[no_mangle]
pub extern "C" fn framebuffer_draw_line(framebuffer: &mut Framebuffer, start: Point2<i32>, end: Point2<i32>, width: u32, v: Color) -> mxcfb_rect {
    let c: common::color = match v {
        Color::Black => common::color::BLACK,
        Color::Gray(v) => common::color::GRAY(v)
    };

    framebuffer.draw_line(start, end, width, c)
}

#[no_mangle]
pub extern "C" fn
