mod framebuffer;
mod lines;

use framebuffer::FrameBuffer;
use lines::draw_polygon_outline;
use raylib::prelude::*;

fn main() {
    let (mut rl, _thread) = raylib::init().size(800, 600).title("Poligon 1").build();

    let width = 800;
    let height = 600;
    let mut fb = FrameBuffer::new(width, height);

    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    fill_polygon(&mut fb, &polygon1, Color::YELLOW);
    draw_polygon_outline(&mut fb, &polygon1, Color::WHITE);

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
