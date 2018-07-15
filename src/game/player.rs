
use ::util::vector::Vector2D;

pub struct Player {
    local_position: Vector2D<i16>
}

impl Player {
    pub fn new() -> Player {
        Player {
            local_position: Vector2D::new(10, 10)
        }
    }

    pub fn local_position(&self) -> &Vector2D<i16> {
        &self.local_position
    }

    pub fn move_local_position(&mut self, x_move: i16, y_move: i16) {
        self.local_position.x += x_move;
        self.local_position.y += y_move;
    }
}