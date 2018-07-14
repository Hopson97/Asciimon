
use ::util::vector::Vector2D;

pub struct Player {
    local_postion: Vector2D<u16>
}

impl Player {
    pub fn new() -> Player {
        Player {
            local_postion: Vector2D::new(0, 0)
        }
    }
}