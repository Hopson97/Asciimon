
use ::util::vector::Vector2D;

pub struct Player {
    position: Vector2D<i32>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vector2D::new(5, 45),
        }
    }

    pub fn position(&self) -> &Vector2D<i32> {
        &self.position
    }

    pub fn move_position(&mut self, x_move: i32, y_move: i32) {
        self.position.x += x_move;
        self.position.y += y_move;
    }
}