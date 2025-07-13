mod framebuffer;
mod lines;

use framebuffer::FrameBuffer;
use lines::draw_polygon_outline;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let mut fb = FrameBuffer::new(width, height);

    let polygon4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37),
        (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214),
        (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    let polygon5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    fill_polygon_with_hole(&mut fb, &polygon4, &polygon5, Color::GREEN);
    draw_polygon_outline(&mut fb, &polygon4, Color::WHITE);
    draw_polygon_outline(&mut fb, &polygon5, Color::WHITE);

    fb.save("out.bmp");
}

fn fill_polygon_with_hole(fb: &mut FrameBuffer, outer: &[(i32, i32)], hole: &[(i32, i32)], color: Color) {
    let min_y = outer.iter().map(|p| p.1).min().unwrap_or(0);
    let max_y = outer.iter().map(|p| p.1).max().unwrap_or(0);

    for y in min_y..=max_y {
        let mut outer_intersections = get_scanline_intersections(outer, y);
        let mut hole_intersections = get_scanline_intersections(hole, y);

        outer_intersections.sort();
        hole_intersections.sort();

        for i in (0..outer_intersections.len()).step_by(2) {
            if i + 1 >= outer_intersections.len() { continue; }
            let x_start = outer_intersections[i];
            let x_end = outer_intersections[i + 1];

            let mut x_ranges = vec![(x_start, x_end)];

            for j in (0..hole_intersections.len()).step_by(2) {
                if j + 1 >= hole_intersections.len() { continue; }
                let h_start = hole_intersections[j];
                let h_end = hole_intersections[j + 1];

                let mut new_ranges = vec![];

                for (r_start, r_end) in x_ranges {
                    if h_end <= r_start || h_start >= r_end {
                        new_ranges.push((r_start, r_end));
                    } else {
                        if h_start > r_start {
                            new_ranges.push((r_start, h_start - 1));
                        }
                        if h_end < r_end {
                            new_ranges.push((h_end + 1, r_end));
                        }
                    }
                }

                x_ranges = new_ranges;
            }

            for (r_start, r_end) in x_ranges {
                for x in r_start..=r_end {
                    fb.set_pixel(x, y, color);
                }
            }
        }
    }
}

fn get_scanline_intersections(polygon: &[(i32, i32)], y: i32) -> Vec<i32> {
    let mut intersections = vec![];

    for i in 0..polygon.len() {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % polygon.len()];

        if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
            let dy = y1 - y0;
            if dy != 0 {
                let dx = x1 - x0;
                let x = x0 + (dx * (y - y0)) / dy;
                intersections.push(x);
            }
        }
    }

    intersections
}
