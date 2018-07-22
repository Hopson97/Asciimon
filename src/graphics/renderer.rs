use super::colour::Colour;

use ::util::vector::Vector2D;

use std::collections::HashMap;

use super::sprite::Sprite;


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
            start_point, 
            size,
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
    
    pub fn add_render_section(&mut self, name: &str, start_point: Vector2D<u8>, size: Vector2D<u8>) {
        self.render_sections.insert(
            name.to_string(), 
            RenderSection::new(start_point, size));
    }

    /*
    *   Functions for crating the user-interface display
    */
    pub fn create_border(&mut self) {
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
        self.clear_section("full", &self.clear_colour);
    }

    pub fn clear_section(&self, section: &'static str, colour: &Colour) {
        Renderer::set_bg_colour(&colour);

        match self.render_sections.get(section) {
            None => { return; },
            Some(render_section) => {
                for y in 0..render_section.size.y {
                    for x in 0..render_section.size.x {
                        self.draw_string(section, " ", &Vector2D::new(x as i16, y as i16));
                    }
                }
            } 
        }
    }

    pub fn default_clear_colour(&self) -> &Colour {
        &self.clear_colour
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
    pub fn set_cursor_render_section(&self, section: &str, position: &Vector2D<u8>) {
        match self.render_sections.get(section) {
            None => panic!(format!("Tried to render to section which doesn't exist: {}", section)),
            Some(section) => {
                Renderer::set_cursor_location(section.start_point.x + position.x + 1, section.start_point.y + position.y + 1);
            }
        }
    }

    pub fn draw_string(&self, section: &str, string: &str, start_position: &Vector2D<i16>) {
        let sect = self.render_sections.get(section).unwrap();

        if start_position.y < 0 || start_position.y >= sect.size.y as i16 {
            return;
        }

        self.set_cursor_render_section(section, &Vector2D::new(start_position.x as u8, start_position.y as u8));
        print!("{}", string);
    }

    pub fn draw_sprite(&self, section: &str, sprite: &Sprite) {
        let position = &sprite.position;
        self.set_cursor_render_section(section, &Vector2D::new(position.x as u8, position.y as u8));
        let data = sprite.render_data();

        let mut line_num = 0;
        for line in data {
            self.draw_string(section, line, &Vector2D::new(position.x, position.y + line_num));
            line_num += 1;
        }
    }


    /*
        Misc functions
    */
    pub fn get_size(&self) -> &Vector2D<u8> {
        &self.size
    }
}
