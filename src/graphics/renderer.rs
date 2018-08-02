use super::colour::Colour;

use util::vector;
use util::vector::Vector2D;

use super::sprite::Sprite;
use std::collections::HashMap;

mod colours {
    use graphics::colour::Colour;
    define_colour!(CLEAR_COLOUR, 25, 20, 70);
    define_colour!(BORDER, 20, 20, 20);
}

pub struct Panel {
    start_point: Vector2D<i32>,
    size: Vector2D<i32>,
}

impl Panel {
    pub fn new(start_point: Vector2D<i32>, size: Vector2D<i32>) -> Panel {
        Panel { start_point, size }
    }
}

pub struct Renderer {
    size: Vector2D<i32>,
    clear_colour: Colour,
    panels: HashMap<String, Panel>,
}

impl Renderer {
    pub fn new(size: Vector2D<i32>) -> Renderer {
        let mut renderer = Renderer {
            size,
            clear_colour: colours::CLEAR_COLOUR,
            panels: HashMap::new(),
        };

        renderer.add_panel("full", Panel::new(vector::ZERO, size));
        renderer.create_border("full");
        renderer.clear_panel("full", &renderer.clear_colour);

        renderer
    }

    pub fn panel(&self, name: &str) -> &Panel {
        &self.panels[name]
    }

    pub fn add_panel(&mut self, name: &str, panel: Panel) {
        self.panels.insert(name.to_string(), panel);
    }

    ///Clears just a single panel
    pub fn clear_panel(&self, panel_name: &str, colour: &Colour) {
        Renderer::set_bg_colour(&colour);

        let panel = self.panel(panel_name);
        for y in 0..panel.size.y {
            self.draw_string(
                panel_name,
                &" ".repeat(panel.size.x as usize),
                Vector2D::new(0, y),
            );
        }
    }

    pub fn size(&self) -> Vector2D<i32> {
        self.size
    }

    ///Draws a solid line in the X-plane of the renderer
    fn draw_solid_line_x(&self, colour: &Colour, begin_position: Vector2D<i32>, length: i32) {
        Renderer::set_bg_colour(colour);
        Renderer::set_cursor_location(begin_position);
        for _x in begin_position.x..length {
            print!(" ");
        }
        Renderer::set_bg_colour(&self.clear_colour);
    }

    ///Draws a solid line in the Y-Plane of the renderer
    fn draw_solid_line_y(&self, colour: &Colour, begin_position: Vector2D<i32>, height: i32) {
        Renderer::set_bg_colour(colour);
        for y in begin_position.y..height {
            Renderer::set_cursor_location(begin_position + Vector2D::new(0, y));
            print!(" ");
        }
        Renderer::set_bg_colour(&self.clear_colour);
    }

    ///Creates a border around the panel area
    pub fn create_border(&self, panel: &str) {
        let panel = &self.panels[panel];
        let bg_col = colours::BORDER;

        let Panel {
            start_point: Vector2D { x, y },
            size: Vector2D { x: w, y: h },
        } = panel;

        //Top
        self.draw_solid_line_x(&bg_col, panel.start_point, w + 2);

        //Left
        self.draw_solid_line_y(&bg_col, panel.start_point, h + 2);

        //Bottom
        self.draw_solid_line_x(&bg_col, Vector2D::new(*x, y + h + 1), w + 2);

        //Right
        self.draw_solid_line_y(&bg_col, Vector2D::new(x + w + 1, *y), h + 2);
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
    pub fn set_cursor_location(pos: Vector2D<i32>) {
        print!("\x1b[{};{}H", pos.y + 1, pos.x + 1);
    }

    /*
     * Public drawing interface
     */
    /// Sets the location of the cursor relative to the top-left of a render panel
    pub fn set_cursor_render_panel(&self, panel_name: &str, position: Vector2D<i32>) {
        let panel = self.panel(panel_name);
        Renderer::set_cursor_location(panel.start_point + position + vector::ONE);
    }

    /// Draws a string to a render panel.
    /// Note: The function does not handle the length of strings going outside of the render panel (for now?)
    pub fn draw_string(&self, panel_name: &str, string: &str, start_position: Vector2D<i32>) {
        let panel = self.panel(panel_name);

        if start_position.y < 0 || start_position.y >= panel.size.y {
            return;
        }

        self.set_cursor_render_panel(panel_name, start_position);
        print!("{}", string);
    }

    // Draws a sprite (duh)
    pub fn draw_sprite(&self, panel: &str, sprite: &Sprite) {
        self.set_cursor_render_panel(panel, sprite.position);
        let data = sprite.render_data();

        for (line_num, line) in data.iter().enumerate() {
            self.draw_string(
                panel,
                line,
                sprite.position + Vector2D::new(0, line_num as i32),
            );
        }
    }
}
