use super::CONSOLE_SIZE;

use graphics::Colour;
use graphics::Renderer;

use std::collections::vec_deque::VecDeque;

use util::Vector2D;

mod colours {
    use graphics::Colour;
    define_colour!(DEFAULT_TEXT, 200, 200, 200);
    define_colour!(BACKGROUND, 0, 0, 5);
}

/// A single section of text to be rendered to the console
struct ConsoleOutputSection {
    text: Vec<String>,
    colour: Colour,
}

/// This is the "console" that displays on the right-side of the user interface
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
        Renderer::set_text_colour(self.colour);

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

    ///Writes a string to the console interface
    pub fn write(&mut self, text: &str) {
        self.write_with_colour(text, colours::DEFAULT_TEXT);
    }

    ///Writes a string to console interface... With colour!
    pub fn write_with_colour(&mut self, text: &str, colour: Colour) {
        let words: Vec<&str> = text.split(' ').collect();
        let mut output_section = ConsoleOutputSection::new(colour);
        let mut current_line = String::with_capacity(CONSOLE_SIZE.x as usize);

        current_line.push_str("> ");
        let mut length = 2;

        for word in &words {
            length += word.len() + 1; //+ 1 for the space after the char

            //Prevents string going over edge of the render section
            if length >= CONSOLE_SIZE.x as usize {
                output_section.add_line(&current_line);
                current_line.clear();
                current_line.push_str("  "); //to clear past the "> " of the first string
                length = word.len() + 3;
            }
            current_line.push_str(&format!("{} ", word));
        }
        output_section.add_line(&current_line);
        self.output_sections.push_front(output_section);
    }

    ///Writes an empty line
    pub fn skip_line(&mut self) {
        self.output_sections
            .push_front(ConsoleOutputSection::new(colours::DEFAULT_TEXT));
    }

    ///Draw all the render sections that can fit, starting with the newest at the top
    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_section("console", colours::BACKGROUND);

        let mut y = 0;
        for (index, line) in self.output_sections.iter().enumerate() {
            line.draw(index + y, &renderer);
            if line.num_texts() > 0 {
                y += line.num_texts() - 1;
            }
        }
    }
}
