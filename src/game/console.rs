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
    text: Vec<String>,
    colour: Colour,
}

///This is the "console" that displays on the right-side of the user interface
pub struct Console {
    output_sections: VecDeque<ConsoleOutputSection>,
}

impl ConsoleOutputSection {
    pub fn new(colour: Colour) -> ConsoleOutputSection {
        ConsoleOutputSection {
            text: Vec::with_capacity(3),
            colour,
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.text.push(line.to_string());
    }

    pub fn draw(&self, index: usize, renderer: &Renderer) {
        Renderer::set_text_colour(&self.colour);

        for (line_num, line) in self.text.iter().enumerate() {
            renderer.draw_string(
                "console",
                &line,
                Vector2D::new(0, (index + line_num) as i32),
            );
        }
    }

    pub fn num_texts(&self) -> usize {
        self.text.len()
    }
}

impl Console {
    pub fn new() -> Console {
        Console {
            output_sections: VecDeque::with_capacity(53),
        }
    }

    pub fn write(&mut self, text: &str) {
        self.write_with_colour(text, colours::DEFAULT_TEXT);
    }

    pub fn write_with_colour(&mut self, text: &str, colour: Colour) {
        let words: Vec<&str> = text.split(' ').collect();
        let mut output_sect = ConsoleOutputSection::new(colour);
        let mut current_line_str = String::with_capacity(CONSOLE_WIDTH as usize);

        current_line_str.push_str("> ");
        let mut length = 2;

        for word in &words {
            length += word.len() + 1; //+ 1 for the space after the char
            if length >= CONSOLE_WIDTH as usize {
                output_sect.add_line(&current_line_str);
                current_line_str.clear();
                current_line_str.push_str("  "); //to clear past the "> " of the first string
                length = word.len() + 3;  
            } 
            current_line_str.push_str(&format!("{} ", word));
        }
        output_sect.add_line(&current_line_str);
        self.output_sections.push_back(output_sect);
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_section("console", &colours::BACKGROUND);

        for (index, line) in self.output_sections.iter().rev().enumerate() {
            line.draw(index, &renderer);
        }
    }
}
