use graphics::renderer::Renderer;

use std::collections::vec_deque::VecDeque;

use util::vector::Vector2D;

mod colours {
    use graphics::colour::Colour;
    define_colour!(TEXT, 150, 150, 175);
    define_colour!(BACKGROUND, 0, 0, 0);
}

pub struct Console {
    lines: VecDeque<String>
}

impl Console {
    pub fn new() -> Console{
        Console {
            lines: VecDeque::with_capacity(53)
        }
    }

    pub fn write(&mut self, string: String) {
        self.lines.push_back(string);
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.clear_section("console", &colours::BACKGROUND);
        Renderer::set_text_colour(&colours::TEXT);

        for (line_num, line) in self.lines.iter().rev().enumerate() {
            renderer.draw_string(
                "console", 
                &format!("> {}", line), 
                Vector2D::new(0, line_num as i32));
        }
    }
}