use std::collections::HashMap;

use graphics::renderer::Renderer;
use util::maths;
use util::vector::Vector2D;

use super::chunk::{Chunk, CHUNK_SIZE};

pub struct World {
    //error_chunk: Chunk,
    chunks: HashMap<Vector2D<i32>, Chunk>,
}

impl World {
    pub fn new() -> World {
        World {
            //error_chunk: Chunk::load_from_name(String::from("error").unwrap(),
            chunks: HashMap::with_capacity(20),
        }
    }

    pub fn render(&mut self, renderer: &Renderer, player_position: Vector2D<i32>) {
        let player_chunk_pos = World::player_to_chunk_position(player_position);

        for y in -1..=1 {
            for x in -1..=1 {
                let chunk_pos = player_chunk_pos + Vector2D::new(x, y);

                //To do: Improve the cotains key followed by the insert.
                if !self.chunks.contains_key(&chunk_pos) {
                    if let Some(chunk) = Chunk::load(chunk_pos) {
                        self.chunks.insert(chunk_pos, chunk);
                    }
                }

                if let Some(chunk) = self.chunks.get(&chunk_pos) {
                    chunk.render(renderer, player_position);
                }
            }
        }
    }

    pub fn get_tile(&self, position: Vector2D<i32>) -> char {
        let chunk_position = World::player_to_chunk_position(position);
        self.chunks.get(&chunk_position).map_or(' ', |chunk| {
            let local_x = maths::repeat(position.x, 0, CHUNK_SIZE.x);
            let local_y = maths::repeat(position.y, 0, CHUNK_SIZE.y);

            chunk.get_tile(local_x, local_y)
        })
    }

    fn player_to_chunk_position(player_position: Vector2D<i32>) -> Vector2D<i32> {
        let Vector2D { x, y } = player_position / CHUNK_SIZE;
        Vector2D::new(
            if player_position.x < 0 { x - 1 } else { x },
            if player_position.y < 0 { y - 1 } else { y },
        )
    }
}
