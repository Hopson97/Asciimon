use super::colour::Colour;

use ::util::vector::Vector2D;


use super::sprite::Sprite;
use std::collections::HashMap;


struct RenderSection {
    start_point: Vector2D<i32>,
    size: Vector2D<i32>,

}

pub struct Renderer {
    size: Vector2D<i32>,
    clear_colour: Colour,
    render_sections: HashMap<String, RenderSection>
}

impl RenderSection {
    pub fn new(start_point: Vector2D<i32>, size: Vector2D<i32>) -> RenderSection {
        RenderSection {
            start_point, 
            size,
        }
    }
}

impl Renderer {
    pub fn new(x_size: i32, y_size: i32) -> Renderer {
        let mut renderer = Renderer {
            size: Vector2D::new(x_size, y_size),
            clear_colour: Colour::new(25, 20, 70),
            render_sections: HashMap::new(),
        };
        renderer.add_render_section("full",  Vector2D::new(0, 0),          Vector2D::new(x_size, y_size));
        renderer.add_render_section("debug", Vector2D::new(x_size + 2, 0), Vector2D::new(20,     y_size));
        renderer.create_border("full");
        renderer.clear();
        renderer
    }
    
    pub fn add_render_section(&mut self, name: &'static str, start_point: Vector2D<i32>, size: Vector2D<i32>) {
        self.render_sections.insert(
            name.to_string(), 
            RenderSection::new(start_point, size));
    }


    ///Clears the entire window
    pub fn clear(&mut self) {
        self.clear_section("full", &self.clear_colour);
    }

    ///Clears just a single section of the screen
    pub fn clear_section(&self, section: &'static str, colour: &Colour) {
        Renderer::set_bg_colour(&colour);

        match self.render_sections.get(section) {
            None => { return; },
            Some(render_section) => {
                for y in 0..render_section.size.y {
                    for x in 0..render_section.size.x {
                        self.draw_string(section, " ", &Vector2D::new(x, y));
                    }
                }
            } 
        }
    }

    ///Gets the default clear colour
    pub fn default_clear_colour(&self) -> &Colour {
        &self.clear_colour
    }

    ///Draws a solid line in the X-plane of the renderer
    fn draw_solid_line_x(&self, colour: &Colour, begin_position: &Vector2D<i32>, length: i32) {
        Renderer::set_bg_colour(colour);
        Renderer::set_cursor_location(begin_position.x, begin_position.y);
        for _x in begin_position.x..length {
            print!(" ");
        }
        Renderer::set_bg_colour(&self.clear_colour);
    }

    ///Draws a solid line in the Y-Plane of the renderer
    fn draw_solid_line_y(&self, colour: &Colour, begin_position: &Vector2D<i32>, height: i32) {
        Renderer::set_bg_colour(colour);
        for y in begin_position.y..height {
            Renderer::set_cursor_location(begin_position.x, begin_position.y + y);
            print!(" ");
        }
        Renderer::set_bg_colour(&self.clear_colour);
    }

    ///Creates a border around the rendering section area
    pub fn create_border(&self, section: &str) {
        let sect = &self.render_sections[section];
        let bg_col = Colour::new(20, 20, 20);

        let x = sect.start_point.x;
        let y = sect.start_point.y;
        let width = sect.size.x;
        let height = sect.size.y;

        //Top
        self.draw_solid_line_x(&bg_col, &sect.start_point, width + 2);

        //Left
        self.draw_solid_line_y(&bg_col, &sect.start_point, height + 2);

        //Bottom
        self.draw_solid_line_x(&bg_col,
            &Vector2D::new(x, y + height + 1), 
            width + 2);

        //Right
        self.draw_solid_line_y(&bg_col, 
            &Vector2D::new(x + width + 1, y), 
            height + 2);
    }
 
    /// Set the foreground colour for text printed to the terminal
    pub fn set_text_colour(colour: &Colour) {
        Renderer::set_colour(38, &colour);
    }
    
    /// Set the background colour for text printed to the terminal
    pub fn set_bg_colour(colour: &Colour) {
        Renderer::set_colour(48, &colour);
    }

    /// Sets either background/foreground colour for text printed to terminal window
    /// # Examples
    /// Renderer::set_color(48, /* some colour here */)
    /// 
    /// This sets the background colour, as 48 is the identifier for background
    fn set_colour(ansi: u8, colour: &Colour) {
        print!("\x1b[{};2;{};{};{}m", 
            ansi, colour.r, colour.g, colour.b);
    }

    /// Sets cursor location in the renderer
    pub fn set_cursor_location(x: i32, y: i32) {
        print!("\x1b[{};{}H", y + 1, x + 1);
    }

    /*
     * Public drawing interface
     */
    /// Sets the location of the cursor relative to the top-left of a render section
    pub fn set_cursor_render_section(&self, section: &str, position: &Vector2D<i32>) {
        match self.render_sections.get(section) {
            None => panic!(format!("Tried to render to section which doesn't exist: {}", section)),
            Some(section) => {
                Renderer::set_cursor_location(section.start_point.x + position.x + 1, section.start_point.y + position.y + 1);
            }
        }
    }

    /// Draws a string to a render section.
    /// Note: The function does not handle the length of strings going outside of the render section (for now?)
    pub fn draw_string(&self, section: &str, string: &str, start_position: &Vector2D<i32>) {
        let sect = match self.render_sections.get(section) {
            None        => panic!("Render section: {} does not exist!", section),
            Some(sect)  => sect
        };

        if start_position.y < 0 || start_position.y >= sect.size.y {
            return;
        }

        self.set_cursor_render_section(section, &Vector2D::new(start_position.x, start_position.y));
        print!("{}", string);
    }

    // Draws a sprite (duh)
    pub fn draw_sprite(&self, section: &str, sprite: &Sprite) {
        let position = &sprite.position;
        self.set_cursor_render_section(section, &Vector2D::new(position.x, position.y));
        let data = sprite.render_data();

        let mut line_num = 0;
        for line in data {
            self.draw_string(section, line, &Vector2D::new(position.x, position.y + line_num));
            line_num += 1;
        }
    }
}
