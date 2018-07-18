use ::util::vector::Vector2D;

pub struct Sprite {
    pub position: Vector2D<u8>,
    lines: Vec<String>
}

impl Sprite {
    pub fn make_empty() -> Sprite {
        Sprite {
            position: Vector2D::new(0, 0),
            lines: Vec::new(),
        }
    }

    pub fn make_square(width: u8, height: u8, fill: char) -> Sprite {
        let mut sprite = Sprite::make_empty();
        for y in 0..height { 
            sprite.lines.push(String::new());
            for x in 0..width {
                sprite.lines.last_mut().unwrap().push(fill);
            }
        }

        sprite
    }

    pub fn render_data(&self) -> &Vec<String> {
        &self.lines
    }
}
