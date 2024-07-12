use crate::colors::Color;
use crate::vertex::Vertex;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub background_color: Color,
    pub foreground_color: Color,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let background_color = Color::from_hex(0x000000); // Default to black
        let foreground_color = Color::from_hex(0xFFFFFF); // Default to white
        let buffer = vec![background_color.to_hex(); width * height];
        FrameBuffer {
            width,
            height,
            buffer,
            background_color,
            foreground_color,
        }
    }

    pub fn clear(&mut self) {
        let color_hex = self.background_color.to_hex();
        self.buffer.fill(color_hex);
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }
        let index = y as usize * self.width + x as usize;
        self.buffer[index] = self.foreground_color.to_hex();
    }

    pub fn get_pixel(&self, x: isize, y: isize) -> Option<Color> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        let index = y as usize * self.width + x as usize;
        Some(Color::from_hex(self.buffer[index]))
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = Color::from_hex(color);
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = Color::from_hex(color);
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        crate::bmp::write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}
