
use ::util::vector::Vector2D;
use super::map::{MAP_WIDTH, MAP_HEIGHT};

pub struct Player {
    position: Vector2D<i32>,
}

impl Player {
    pub fn new() -> Player {
        let start_x = 10;
        let start_y = 10;

        let local_x = 25;
        let local_y = 10;
        
        Player {
            position: Vector2D::new(
                start_x * MAP_WIDTH - start_x + local_x, 
                MAP_HEIGHT * start_y - start_y + local_y
            ),
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