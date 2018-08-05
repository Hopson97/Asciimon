use super::colour::Colour;
use super::panel::Panel;
use util::vector;
use util::vector::Vector2D;

use std::collections::HashMap;

pub mod colours {
    use graphics::colour::Colour;
    define_colour!(CLEAR_COLOUR, 25, 20, 70);
    define_colour!(BORDER, 20, 20, 20);
}

pub struct Renderer {
    size: Vector2D<u32>,
    panels: HashMap<String, Panel>,
}

impl Renderer {
    pub fn new(size: Vector2D<u32>) -> Renderer {
        let mut renderer = Renderer {
            size,
            panels: HashMap::new(),
        };

        let main_panel = Panel::new(vector::ZERO, size);
        main_panel.clear(&colours::CLEAR_COLOUR);
        main_panel.border();

        let debug_panel = Panel::new(Vector2D::new(size.x + 2, 0), Vector2D::new(20, size.y));
        debug_panel.clear(&colours::CLEAR_COLOUR);
        debug_panel.border();

        renderer.add_panel("main", main_panel);
        renderer.add_panel("debug", debug_panel);

        renderer
    }

    pub fn panel(&self, name: &str) -> &Panel {
        &self.panels[name]
    }

    pub fn add_panel(&mut self, name: &str, panel: Panel) {
        self.panels.insert(name.to_string(), panel);
    }

    /// Set the foreground colour for text printed to the terminal
    pub fn set_text_colour(colour: &Colour) {
        print!("{}", colour.ansi_text_string());
    }

    /// Set the background colour for text printed to the terminal
    pub fn set_bg_colour(colour: &Colour) {
        print!("{}", colour.ansi_bg_string());
    }

    /// Sets cursor location in the renderer
    pub fn set_cursor_location(pos: Vector2D<u32>) {
        print!("\x1b[{};{}H", pos.y + 1, pos.x + 1);
    }
}
