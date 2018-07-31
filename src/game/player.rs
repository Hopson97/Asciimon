use super::chunk::CHUNK_SIZE;
use util::vector::Vector2D;

pub struct Player {
    pub position: Vector2D<i32>,
}

impl Player {
    pub fn new() -> Player {
        let local_pos = Vector2D::new(20, 25);

        Player {
            position: CHUNK_SIZE * 1000 + local_pos,
        }
    }

    pub fn move_position(&mut self, movement: Vector2D<i32>) {
        self.position += movement;
    }
}
