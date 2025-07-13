use raylib::prelude::*;
use crate::framebuffer::FrameBuffer;

pub fn draw_polygon_outline(fb: &mut FrameBuffer, points: &[(i32, i32)], color: Color) {
    let len = points.len();
    for i in 0..len {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % len];
        fb.draw_line(x0, y0, x1, y1, color);
    }
}
