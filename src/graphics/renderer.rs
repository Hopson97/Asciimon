use super::colour::Colour;

pub struct Renderer {
    x_size: u8,
    y_size: u8,


}

impl Renderer {
    pub fn new(x_size: u8, y_size: u8) -> Renderer {
        let mut r = Renderer {
            x_size, y_size
        };
        r.create_border();

        for x in 0..x_size {
            for y in 0..y_size {
                Renderer::set_cursor_location(x + 1, y + 1);
                print!("#");
            }
        }

        r 
    }

    fn create_border(&mut self) {
        Renderer::set_bg_colour(&Colour::new(0, 0, 0));
        for x in 0..self.x_size + 2 {
            Renderer::set_cursor_location(x, 0);
            print!(" ");
            Renderer::set_cursor_location(x, self.y_size + 1);
            print!(" ");
        }
        for y in 0..self.y_size + 2 {
            Renderer::set_cursor_location(0, y);
            print!(" ");
            Renderer::set_cursor_location(self.x_size + 1, y);
            print!(" ");
        }
    }

    /**
     * Colour functions for changing text colour in the terminal
     */
    fn set_text_colour(colour: &Colour) {
        Renderer::set_colour(38, &colour);
    }
    
    fn set_bg_colour(colour: &Colour) {
        Renderer::set_colour(48, &colour);
    }

    fn set_colour(ansi: u8, colour: &Colour) {
        print!("\x1b[{};2;{};{};{}m", 
            ansi, colour.r, colour.g, colour.b);
    }

    /**
     * Misc ANSI commands
     */
    fn set_cursor_location(x: u8, y: u8) {
        print!("\x1b[{};{}H", y + 1, x + 1);
    }
}