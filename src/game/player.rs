
use ::util::vector::Vector2D;

pub struct Player {
    world_position: Vector2D<i16>
}

impl Player {
    pub fn new() -> Player {
        Player {
            world_position: Vector2D::new(0, 0)
        }
    }

    pub fn position(&self) -> &Vector2D<i16> {
        &self.world_position
    }

    pub fn move_position(&mut self, x_move: i16, y_move: i16) {
        self.world_position.x += x_move;
        self.world_position.y += y_move;
    }
}