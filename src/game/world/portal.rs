use super::chunk::CHUNK_SIZE;
use util::Vector2D;

pub struct Portal {
    world_destination: Vector2D<i32>,
    local_destination: Vector2D<i32>,
}

impl Portal {
    pub fn new(world_destination: Vector2D<i32>, local_destination: Vector2D<i32>) -> Portal {
        Portal {
            world_destination,
            local_destination,
        }
    }

    pub fn get_destination_point(&self) -> Vector2D<i32> {
        let world_tl = Vector2D::new(
            self.world_destination.x * CHUNK_SIZE.x,
            self.world_destination.y * CHUNK_SIZE.y,
        );
        world_tl + self.local_destination
    }
}
