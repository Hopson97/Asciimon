use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour {
            r, g, b
        }
    }

    pub fn ansi_text_colour_string(r: u8, g: u8, b: u8) -> String {
        Colour::colour_string(38, r, g, b)
    }

    pub fn ansi_bg_colour_string(r: u8, g: u8, b: u8) -> String {
        Colour::colour_string(48, r, g, b)
    }

    fn colour_string(id: u8, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[{};2;{};{};{}m", id, r, g, b)
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f32) -> Colour {
        Colour::new(
            (self.r as f32 * rhs) as u8, 
            (self.g as f32 * rhs) as u8, 
            (self.b as f32 * rhs) as u8)
    }
}