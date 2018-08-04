use graphics::colour::Colour;
use graphics::renderer::Renderer;

use std::collections::vec_deque::VecDeque;

use util::vector::Vector2D;

mod colours {
    use graphics::colour::Colour;

    define_colour!(DEFAULT_TEXT, 200, 200, 200);
    define_colour!(BACKGROUND, 0, 0, 5);
}

pub const CONSOLE_WIDTH: i32 = 32;

struct ConsoleOutputSection {
    text: String,
    colour: Colour,
}

pub struct Console {
    output_sections: VecDeque<ConsoleOutputSection>,
}

impl ConsoleOutputSection {
    pub fn new(colour: Colour) -> ConsoleOutputSection {
        ConsoleOutputSection {
            text: String::with_capacity(CONSOLE_WIDTH as usize),
            colour: colour,
        }
    }


    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn colour(&self) -> &Colour {
        &self.colour
    }
}

impl Console {
    pub fn new() -> Console {
        Console {
            output_sections: VecDeque::with_capacity(53),
        }
    }

    pub fn write(&mut self, text: String) {
        let words: Vec<&str> = text.split(" ").collect();
        let mut len = 0;
        let mut current_line_str = String::with_capacity(CONSOLE_WIDTH as usize);
        current_line_str.push_str("> ");

        for word in &words {
            len += word.len();
            if len >= (CONSOLE_WIDTH - 2) as usize {
                 
            } else {
                current_line_str.push_str(word);
            }
        }



        /*
        if text.len() as i32 > CONSOLE_WIDTH {
            let line = &text[0..CONSOLE_WIDTH as usize];
            let line1 = &text[CONSOLE_WIDTH as usize..text.len() - 1];

            self.lines.push_back(Line::new(line.to_string(), colours::DEFAULT_TEXT));
            self.lines.push_back(Line::new(line1.to_string(), colours::DEFAULT_TEXT));

        }
        else {
             self.lines.push_back(Line::new(text, colours::DEFAULT_TEXT));
        }*/
    }

    pub fn write_with_colour(&mut self, text: String, colour: Colour) {
        //self.lines.push_back(Line::new(text, colour));
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_section("console", &colours::BACKGROUND);

        for (line_num, line) in self.output_sections.iter().rev().enumerate() {
            Renderer::set_text_colour(&line.colour());
            renderer.draw_string(
                "console",
                &format!("> {}", &line.text()),
                Vector2D::new(0, line_num as i32),
            );
        }
    }
}
