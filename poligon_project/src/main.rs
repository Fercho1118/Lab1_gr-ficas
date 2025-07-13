mod framebuffer;
mod lines;

use framebuffer::FrameBuffer;
use lines::draw_polygon_outline;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut fb = FrameBuffer::new(width, height);

    let polygon2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    fill_polygon(&mut fb, &polygon2, Color::BLUE); 
    draw_polygon_outline(&mut fb, &polygon2, Color::WHITE); 

    fb.save("out.bmp");
}

fn fill_polygon(fb: &mut FrameBuffer, points: &[(i32, i32)], color: Color) {
    let min_y = points.iter().map(|p| p.1).min().unwrap_or(0);
    let max_y = points.iter().map(|p| p.1).max().unwrap_or(0);

    for y in min_y..=max_y {
        let mut intersections = vec![];

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let dy = y1 - y0;
                if dy != 0 {
                    let dx = x1 - x0;
                    let x = x0 + (dx * (y - y0)) / dy;
                    intersections.push(x);
                }
            }
        }

        intersections.sort();
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                for x in intersections[i]..=intersections[i + 1] {
                    fb.set_pixel(x, y, color);
                }
            }
        }
    }
}
