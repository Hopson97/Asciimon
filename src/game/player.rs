
use ::util::vector::Vector2D;

pub struct Player {
    local_position: Vector2D<u8>
}

impl Player {
    pub fn new() -> Player {
        Player {
            local_position: Vector2D::new(10, 10)
        }
    }

    pub fn get_local_position(&self) -> &Vector2D<u8> {
        &self.local_position
    }
}