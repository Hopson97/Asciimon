use super::colour::Colour;

use util::{vector, Vector2D};

use super::sprite::Sprite;
use std::collections::HashMap;

mod colours {
    use graphics::Colour;
    define_colour!(CLEAR_COLOUR, 25, 20, 70);
    define_colour!(BORDER, 20, 20, 20);
}

struct RenderSection {
    start_point: Vector2D<i32>,
    size: Vector2D<i32>,
}

pub struct Renderer {
    size: Vector2D<i32>,
    clear_colour: Colour,
    render_sections: HashMap<String, RenderSection>,
}

impl RenderSection {
    pub fn new(start_point: Vector2D<i32>, size: Vector2D<i32>) -> RenderSection {
        RenderSection { start_point, size }
    }
}

impl Renderer {
    pub fn new(size: Vector2D<i32>) -> Renderer {
        let mut renderer = Renderer {
            size,
            clear_colour: colours::CLEAR_COLOUR,
            render_sections: HashMap::new(),
        };
        renderer.add_render_section("full", Vector2D::new(0, 0), size);
        renderer.create_border("full");
        renderer.clear();
        renderer
    }

    pub fn size(&self) -> Vector2D<i32> {
        self.size
    }

    /// Adds a section to the UI to draw things to
    pub fn add_render_section(
        &mut self,
        name: &'static str,
        start_point: Vector2D<i32>,
        size: Vector2D<i32>,
    ) {
        self.render_sections
            .insert(name.to_string(), RenderSection::new(start_point, size));
        self.create_border(name);
    }

    /// Clears the entire window
    pub fn clear(&mut self) {
        self.clear_section("full", self.clear_colour);
    }

    /// Clears just a single section of the screen
    pub fn clear_section(&self, section: &'static str, colour: Colour) {
        Renderer::set_bg_colour(colour);

        match self.render_sections.get(section) {
            None => {
                return;
            }
            Some(render_section) => {
                for y in 0..=render_section.size.y {
                    self.draw_string(
                        section,
                        &" ".repeat(render_section.size.x as usize),
                        Vector2D::new(0, y),
                    );
                }
            }
        }
    }

    /// Draws a solid horizontal line
    fn draw_line_h(&self, colour: Colour, begin_position: Vector2D<i32>, length: i32) {
        Renderer::set_bg_colour(colour);
        Renderer::set_cursor_location(begin_position);
        for _x in 0..length {
            print!(" ");
        }
        Renderer::set_bg_colour(self.clear_colour);
    }

    /// Draws a solid vertical line
    fn draw_line_v(&self, colour: Colour, begin_position: Vector2D<i32>, height: i32) {
        Renderer::set_bg_colour(colour);
        for y in 0..height {
            Renderer::set_cursor_location(begin_position + Vector2D::new(0, y));
            print!(" ");
        }
        Renderer::set_bg_colour(self.clear_colour);
    }

    /// Creates a border around the rendering section area
    pub fn create_border(&self, section: &str) {
        let sect = &self.render_sections[section];
        let bg_col = colours::BORDER;

        let Vector2D { x: w, y: h } = sect.size;

        // top
        self.draw_line_h(bg_col, sect.start_point, w + 2);
        // left
        self.draw_line_v(bg_col, sect.start_point - Vector2D::new(1, 0), h + 2);
        self.draw_line_v(bg_col, sect.start_point, h + 2);
        // bottom
        self.draw_line_h(bg_col, sect.start_point + Vector2D::new(0, h + 1), w + 2);
        // right
        self.draw_line_v(bg_col, sect.start_point + Vector2D::new(w + 1, 0), h + 2);
        self.draw_line_v(bg_col, sect.start_point + Vector2D::new(w + 2, 0), h + 2);
    }

    /// Set the foreground colour for text printed to the terminal
    pub fn set_text_colour(colour: Colour) {
        print!("{}", colour.ansi_text_string());
    }

    /// Set the background colour for text printed to the terminal
    pub fn set_bg_colour(colour: Colour) {
        print!("{}", colour.ansi_bg_string());
    }

    /// Sets cursor location in the renderer
    pub fn set_cursor_location(pos: Vector2D<i32>) {
        print!("\x1b[{};{}H", pos.y + 1, pos.x + 2);
    }

    /*
     * Public drawing interface
     */
    /// Sets the location of the cursor relative to the top-left of a render section
    pub fn set_cursor_render_section(&self, section: &str, position: Vector2D<i32>) {
        match self.render_sections.get(section) {
            None => panic!(format!(
                "Tried to render to section which doesn't exist: {}",
                section
            )),
            Some(section) => {
                Renderer::set_cursor_location(section.start_point + position + vector::ONE);
            }
        }
    }

    /// Draws a string to a render section.
    /// Note: The function does not handle the length of strings going outside of the render section (for now?)
    pub fn draw_string(&self, section: &str, string: &str, start_position: Vector2D<i32>) {
        let sect = match self.render_sections.get(section) {
            None => panic!("Render section: {} does not exist!", section),
            Some(sect) => sect,
        };

        if start_position.y < 0 || start_position.y >= sect.size.y {
            return;
        }

        self.set_cursor_render_section(section, start_position);
        print!("{}", string);
    }

    // Draws a sprite (duh)
    pub fn draw_sprite(&self, section: &str, sprite: &Sprite) {
        self.set_cursor_render_section(section, sprite.position);
        let data = sprite.render_data();

        for (line_num, line) in data.iter().enumerate() {
            self.draw_string(
                section,
                line,
                sprite.position + Vector2D::new(0, line_num as i32),
            );
        }
    }
}
