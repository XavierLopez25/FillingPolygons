use crate::framebuffer::FrameBuffer;
use crate::vertex::Vertex;

pub trait Line {
    fn line(&mut self, v1: Vertex, v2: Vertex);
    fn draw_polygon(&mut self, vertices: &[Vertex]);
}

impl Line for FrameBuffer {
    fn line(&mut self, v1: Vertex, v2: Vertex) {
        let (x1, y1) = (v1.x.round() as isize, v1.y.round() as isize);
        let (x2, y2) = (v2.x.round() as isize, v2.y.round() as isize);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;

        let mut x = x1;
        let mut y = y1;

        loop {
            if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
                self.point(x, y);
            }
            if x == x2 && y == y2 { break; }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_polygon(&mut self, vertices: &[Vertex]) {
        if vertices.len() < 2 {
            return;
        }
        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = vertices[(i + 1) % vertices.len()]; // The next point, wrapping around to the start
            self.line(start, end);
        }
    }
}
