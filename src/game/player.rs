
use ::util::vector::Vector2D;

pub struct Player {
    position: Vector2D<i16>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vector2D::new(5, 45),
        }
    }

    pub fn position(&self) -> &Vector2D<i16> {
        &self.position
    }

    pub fn move_position(&mut self, x_move: i16, y_move: i16) {
        self.position.x += x_move;
        self.position.y += y_move;
    }
}