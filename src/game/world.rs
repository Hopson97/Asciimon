use std::collections::HashMap;

use graphics::renderer::Renderer;
use util::maths;
use util::vector::Vector2D;

use super::chunk::{Chunk, CHUNK_SIZE};
use super::{GAME_AREA_CENTER, GAME_AREA_SIZE};

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
        let Vector2D { x, y } = player_chunk_pos;

        for chunk_y in (y - 1)..=(y + 1) {
            for chunk_x in (x - 1)..=(x + 1) {
                let pos = Vector2D::new(chunk_x, chunk_y);

                //To do: Improve the cotains key followed by the insert.
                if !self.chunks.contains_key(&pos) {
                    if let Some(chunk) = Chunk::load(chunk_x, chunk_y) {
                        self.chunks.insert(pos, chunk);
                    }
                }
            }
        }

        for chunk in self.chunks.values() {
            World::render_chunk(&renderer, &chunk, player_position);
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

    fn render_chunk(renderer: &Renderer, chunk: &Chunk, player_position: Vector2D<i32>) {
        //Top left position of where the chunk is drawn from
        let mut chunk_pos = GAME_AREA_CENTER - player_position + chunk.world_position * CHUNK_SIZE;

        //Don't try draw chunk if it is outside of the bounds of the game rendering area
        if chunk_pos.x > GAME_AREA_SIZE.x || chunk_pos.y + CHUNK_SIZE.x < 0 {
            return;
        }

        //String slice of where the chunk lines are drawn from and to
        let mut begin_slice = 0;
        let mut end_slice = CHUNK_SIZE.x - 1;

        if chunk_pos.x < 0 {
            begin_slice = chunk_pos.x.abs();
            chunk_pos.x = 0;
        }

        if chunk_pos.x + (end_slice - begin_slice) > GAME_AREA_SIZE.x as i32 {
            end_slice = (GAME_AREA_SIZE.x as i32 - chunk_pos.x) + begin_slice;
        }

        for y in 0..CHUNK_SIZE.y {
            chunk.draw_line(
                renderer,
                y as usize,
                begin_slice as usize,
                end_slice as usize,
                chunk_pos + Vector2D::new(0, y),
            );
        }
    }

    fn player_to_chunk_position(player_position: Vector2D<i32>) -> Vector2D<i32> {
        let Vector2D { x, y } = player_position / CHUNK_SIZE;
        Vector2D::new(
            if player_position.x < 0 { x - 1 } else { x },
            if player_position.y < 0 { y - 1 } else { y },
        )
    }
}
