use graphics::renderer::Renderer;
use graphics::colour::Colour;

use std::collections::vec_deque::VecDeque;

use util::vector::Vector2D;

mod colours {
    use graphics::colour::Colour;

    define_colour!(DEFAULT_TEXT, 200, 200, 200);
    define_colour!(BACKGROUND, 0, 0, 5);
}

struct Line {
    text: String,
    colour: Colour
}

pub struct Console {
    lines: VecDeque<Line>
}

impl Line {
    pub fn new(text: String, colour: Colour) -> Line {
        Line {
            text: text,
            colour: colour
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
    pub fn new() -> Console{
        Console {
            lines: VecDeque::with_capacity(53)
        }
    }

    pub fn write(&mut self, text: String) {
        self.lines.push_back(Line::new(text, colours::DEFAULT_TEXT));
    }

    pub fn write_with_colour(&mut self, text: String, colour: Colour) {
        self.lines.push_back(Line::new(text, colour));
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_section("console", &colours::BACKGROUND);

        for (line_num, line) in self.lines.iter().rev().enumerate() {
            Renderer::set_text_colour(&line.colour());
            renderer.draw_string(
                "console", 
                &format!("> {}", &line.text()), 
                Vector2D::new(0, line_num as i32));
        }
    }
}