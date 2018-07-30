use super::map::{MAP_HEIGHT, MAP_WIDTH};
use util::vector::Vector2D;

pub struct Player {
    pub position: Vector2D<i32>,
}

impl Player {
    pub fn new() -> Player {
        let local_x = 20;
        let local_y = 25;

        Player {
            position: Vector2D::new(1000 * MAP_WIDTH + local_x, 1000 * MAP_HEIGHT + local_y),
        }
    }

    pub fn move_position(&mut self, movement: Vector2D<i32>) {
        self.position += movement;
    }
}
