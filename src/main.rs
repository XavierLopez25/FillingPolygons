mod colors;
mod framebuffer;
mod bmp;
mod line_impl;
mod vertex;

use crate::framebuffer::FrameBuffer;
use crate::line_impl::Line;
use crate::vertex::Vertex;
use nalgebra_glm as glm;

fn main() {
    let mut framebuffer = FrameBuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Define the colors for the first polygon
    let border_color1 = 0xFFFFFF; // White
    let fill_color1 = 0xCCCC00;   // Yellow

    // Define the vertices of the first polygon
    let vertices1 = vec![
        glm::vec3(165.0, 380.0, 0.0),
        glm::vec3(185.0, 360.0, 0.0),
        glm::vec3(180.0, 330.0, 0.0),
        glm::vec3(207.0, 345.0, 0.0),
        glm::vec3(233.0, 330.0, 0.0),
        glm::vec3(230.0, 360.0, 0.0),
        glm::vec3(250.0, 380.0, 0.0),
        glm::vec3(220.0, 385.0, 0.0),
        glm::vec3(205.0, 410.0, 0.0),
        glm::vec3(193.0, 383.0, 0.0),
    ];

    // Define the colors for the second polygon
    let border_color2 = 0xFFFFFF; // White
    let fill_color2 = 0x0000FF;   // Blue

    // Define the vertices of the second polygon
    let vertices2 = vec![
        glm::vec3(321.0, 335.0, 0.0),
        glm::vec3(288.0, 286.0, 0.0),
        glm::vec3(339.0, 251.0, 0.0),
        glm::vec3(374.0, 302.0, 0.0),
    ];

    // Define the colors for the third polygon
    let border_color3 = 0xFFFFFF; // White
    let fill_color3 = 0xFF0000;   // Red

    // Define the vertices of the third polygon
    let vertices3 = vec![
        glm::vec3(377.0, 249.0, 0.0),
        glm::vec3(411.0, 197.0, 0.0),
        glm::vec3(436.0, 249.0, 0.0),
    ];

    // Define the colors for the fourth polygon
    let border_color4 = 0xFFFFFF; // White
    let fill_color4 = 0x00FF00;   // Green

    // Define the vertices of the fourth polygon
    let vertices4 = vec![
        glm::vec3(413.0, 177.0, 0.0),
        glm::vec3(448.0, 159.0, 0.0),
        glm::vec3(502.0, 88.0, 0.0),
        glm::vec3(553.0, 53.0, 0.0),
        glm::vec3(535.0, 36.0, 0.0),
        glm::vec3(676.0, 37.0, 0.0),
        glm::vec3(660.0, 52.0, 0.0),
        glm::vec3(750.0, 145.0, 0.0),
        glm::vec3(761.0, 179.0, 0.0),
        glm::vec3(672.0, 192.0, 0.0),
        glm::vec3(659.0, 214.0, 0.0),
        glm::vec3(615.0, 214.0, 0.0),
        glm::vec3(632.0, 230.0, 0.0),
        glm::vec3(580.0, 230.0, 0.0),
        glm::vec3(597.0, 215.0, 0.0),
        glm::vec3(552.0, 214.0, 0.0),
        glm::vec3(517.0, 144.0, 0.0),
        glm::vec3(466.0, 180.0, 0.0),
    ];

    // Draw and fill the first polygon
    draw_polygon(&mut framebuffer, &vertices1, border_color1, fill_color1);

    // Draw and fill the second polygon
    draw_polygon(&mut framebuffer, &vertices2, border_color2, fill_color2);

    // Draw and fill the third polygon
    draw_polygon(&mut framebuffer, &vertices3, border_color3, fill_color3);

    // Draw and fill the fourth polygon
    draw_polygon(&mut framebuffer, &vertices4, border_color4, fill_color4);

    framebuffer.render_buffer("output.bmp").expect("Failed to write BMP file");

    println!("Framebuffer rendered to output.bmp");
}

fn draw_polygon(framebuffer: &mut FrameBuffer, vertices: &[Vertex], border_color: u32, fill_color: u32) {
    if vertices.len() < 3 {
        return; 
    }

    framebuffer.set_foreground_color(fill_color);
    fill_polygon(framebuffer, vertices);

    framebuffer.set_foreground_color(border_color);
    framebuffer.draw_polygon(vertices);
}

fn fill_polygon(framebuffer: &mut FrameBuffer, vertices: &[Vertex]) {
    // Find the min and max y values
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min).round() as isize;
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max).round() as isize;

    for y in min_y..=max_y {
        let mut intersections = Vec::new();
        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];

            if (v1.y <= y as f32 && v2.y > y as f32) || (v2.y <= y as f32 && v1.y > y as f32) {
                let x = v1.x + (y as f32 - v1.y) * (v2.x - v1.x) / (v2.y - v1.y);
                intersections.push(x.round() as isize);
            }
        }

        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x1 = intersections[i];
                let x2 = intersections[i + 1];

                for x in x1..=x2 {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
