use util::Vector2D;

pub struct Player {
    position: Vector2D<i32>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vector2D::new(20, 25),
        }
    }

    pub fn move_position(&mut self, movement: Vector2D<i32>) {
        self.position += movement;
    }

    pub fn position(&self) -> Vector2D<i32> {
        self.position
    }
}
