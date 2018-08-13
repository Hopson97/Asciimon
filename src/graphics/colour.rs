use std::fmt;
use termion::color as termion;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[macro_export]
macro_rules! define_colour {
    ($name:ident, $r:expr, $g:expr, $b:expr) => {
        pub const $name: Colour = Colour {
            r: $r,
            g: $g,
            b: $b,
        };
    };
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }

    #[cfg(not(feature = "256colour"))]
    fn to_termion(&self) -> impl termion::Color {
        termion::Rgb(self.r, self.g, self.b)
    }

    #[cfg(feature = "256colour")]
    fn to_termion(&self) -> impl termion::Color {
        let r = self.r as u16 * 6 / 256;
        let g = self.g as u16 * 6 / 256;
        let b = self.b as u16 * 6 / 256;
        termion::AnsiValue((16 + r * 36 + g * 6 + b) as u8)
    }
}

impl termion::Color for Colour {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_termion().write_fg(f)
    }

    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_termion().write_bg(f)
    }
}
