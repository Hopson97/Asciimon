use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }

    pub fn ansi_text_string(&self) -> String {
        self.ansi_string(38)
    }

    pub fn ansi_bg_string(&self) -> String {
        self.ansi_string(48)
    }

    fn ansi_string(&self, id: u8) -> String {
        format!("\x1b[{};2;{};{};{}m", id, self.r, self.g, self.b)
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f32) -> Colour {
        Colour::new(
            (f32::from(self.r) * rhs) as u8,
            (f32::from(self.g) * rhs) as u8,
            (f32::from(self.b) * rhs) as u8,
        )
    }
}
