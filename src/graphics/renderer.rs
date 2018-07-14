use super::colour::Colour;

use ::util::vector::Vector2D;

use std::collections::HashMap;


struct RenderSection {
    start_point: Vector2D<u8>,
    size: Vector2D<u8>,
}

pub struct Renderer {
    size: Vector2D<u8>,
    clear_colour: Colour,
    render_sections: HashMap<String, RenderSection>
}

impl RenderSection {
    pub fn new(start_point: Vector2D<u8>, size: Vector2D<u8>) -> RenderSection {
        RenderSection {
            start_point, size
        }
    }
}

impl Renderer {
    pub fn new(x_size: u8, y_size: u8) -> Renderer {
        let mut renderer = Renderer {
            size: Vector2D::new(x_size, y_size),
            clear_colour: Colour::new(25, 20, 70),
            render_sections: HashMap::new(),
        };
        renderer.add_render_section("full",     Vector2D::new(0, 0),            Vector2D::new(x_size,   y_size));
        renderer.add_render_section("debug",    Vector2D::new(x_size + 2, 0),   Vector2D::new(20,       y_size));

        renderer.create_border();


        renderer
    }

    pub fn add_render_section(&mut self, name: &'static str, start_point: Vector2D<u8>, size: Vector2D<u8>) {
        self.render_sections.insert(name.to_string(), RenderSection::new(start_point, size));
    }

    /*
    *   Functions for crating the user-interface display
    */
    fn create_border(&mut self) {
        Renderer::set_bg_colour(&Colour::new(20, 20, 20));
        for x in 0..self.size.x + 2 {
            Renderer::set_cursor_location(x, 0);
            print!(" ");
            Renderer::set_cursor_location(x, self.size.y + 1);
            print!(" ");
        }
        for y in 0..self.size.y + 2 {
            Renderer::set_cursor_location(0, y);
            print!(" ");
            Renderer::set_cursor_location(self.size.x + 1, y);
            print!(" ");
        }
    }

    pub fn clear(&mut self) {
        self.clear_section("full");
    }

    pub fn clear_section(&self, section: &'static str) {
        Renderer::set_bg_colour(&self.clear_colour);

        match self.render_sections.get(section) {
            None => {} //Impossible to reach as would have already happened from the set_cursor_render_section function
            Some(render_section) => {
                for y in 0..render_section.size.y {
                    for x in 0..render_section.size.x {
                        self.draw_string(section, " ", &Vector2D::new(x, y));
                    }
                }
            } 
        }
    }

    pub fn draw_solid_line_x(&self, colour: &Colour, begin_position: &Vector2D<u8>, length: u8) {
        Renderer::set_bg_colour(colour);
        Renderer::set_cursor_location(begin_position.x + 1, begin_position.y + 1);
        for _x in begin_position.x..length {
            print!(" ");
        }
        Renderer::set_bg_colour(&self.clear_colour);
    }

    pub fn draw_solid_line_y(&self, colour: &Colour, begin_position: &Vector2D<u8>, height: u8) {
        Renderer::set_bg_colour(colour);
        for y in begin_position.y..height {
            Renderer::set_cursor_location(begin_position.x + 1, begin_position.y + y + 1);
            print!(" ");
        }
        Renderer::set_bg_colour(&self.clear_colour);
    }

    /*
     * Colour functions for changing text colour in the terminal
     */
    pub fn set_text_colour(colour: &Colour) {
        Renderer::set_colour(38, &colour);
    }
    
    pub fn set_bg_colour(colour: &Colour) {
        Renderer::set_colour(48, &colour);
    }

    fn set_colour(ansi: u8, colour: &Colour) {
        print!("\x1b[{};2;{};{};{}m", 
            ansi, colour.r, colour.g, colour.b);
    }

    /*
     * Misc ANSI commands
     */
    pub fn set_cursor_location(x: u8, y: u8) {
        print!("\x1b[{};{}H", y + 1, x + 1);
    }


    /*
     * Public drawing interface
     * self is used to ensure these functions are only called on the object itself and not globally
     */
    pub fn set_cursor_render_section(&self, section: &'static str, position: &Vector2D<u8>) {
        match self.render_sections.get(section) {
            None => panic!(format!("Tried to render to section which doesn't exist: {}", section)),
            Some(section) => {
                Renderer::set_cursor_location(section.start_point.x + position.x + 1, section.start_point.y + position.y + 1);
            }
        }
    }

    pub fn draw_string(&self, section: &'static str, string: &str, start_position: &Vector2D<u8>) {
        self.set_cursor_render_section(section, &start_position);
        print!("{}", string);
    }


    /*
        Misc functions
    */
    pub fn get_size(&self) -> &Vector2D<u8> {
        &self.size
    }
}
