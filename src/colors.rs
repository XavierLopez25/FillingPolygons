#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b }
    }

    pub fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }

    pub fn mul(self, factor: f32) -> Color {
        Color {
            r: ((self.r as f32 * factor).clamp(0.0, 255.0) as u8),
            g: ((self.g as f32 * factor).clamp(0.0, 255.0) as u8),
            b: ((self.b as f32 * factor).clamp(0.0, 255.0) as u8),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_within_range() {
        let color = Color::new(100, 150, 200);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex(0xFF5733);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 87);
        assert_eq!(color.b, 51);
    }

    #[test]
    fn test_add_colors() {
        let color1 = Color::new(100, 150, 200);
        let color2 = Color::new(100, 100, 100);
        let result = color1.add(color2);
        assert_eq!(result.r, 200);
        assert_eq!(result.g, 250);
        assert_eq!(result.b, 255);
    }

    #[test]
    fn test_mul_color() {
        let color = Color::new(100, 100, 100);
        let result = color.mul(3.0);
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 255); 
        assert_eq!(result.b, 255); 
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(255, 87, 51);
        let hex = color.to_hex();
        assert_eq!(hex, 0xFF5733);
    }
}
