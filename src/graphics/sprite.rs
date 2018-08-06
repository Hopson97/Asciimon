use util::{vector, Vector2D};

pub struct Sprite {
    pub position: Vector2D<i32>,
    lines: Vec<String>,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            position: vector::ZERO,
            lines: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Sprite {
        Sprite {
            position: vector::ZERO,
            lines: Vec::with_capacity(capacity),
        }
    }

    pub fn square(width: i32, height: i32, fill: char) -> Sprite {
        let mut sprite = Sprite::new();
        for _y in 0..height {
            sprite.lines.push(String::new());
            for _x in 0..width {
                sprite.lines.last_mut().unwrap().push(fill);
            }
        }

        sprite
    }

    pub fn render_data(&self) -> &Vec<String> {
        &self.lines
    }
}
