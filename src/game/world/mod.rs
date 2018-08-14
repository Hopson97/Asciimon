pub mod chunk;
mod chunk_loader;

use std::collections::HashMap;
use graphics::Renderer;
use util::Vector2D;


pub use self::chunk::{Chunk, CHUNK_SIZE};
use self::chunk_loader::{ChunkLoadData, load_chunk, extract_pos_from_path};

use std::fs::read_dir;


/// World
/// Manages the map chunks within the world
pub struct World {
    //error_chunk: Chunk,
    chunks: HashMap<Vector2D<i32>, Chunk>,
}

impl World {
    pub fn new() -> World {
        let world = World {
            //error_chunk: Chunk::load_from_name(String::from("error").unwrap(),
            chunks: HashMap::with_capacity(20),
        };

        let mut portal_ids: Vec<(i32, Vector2D<i32>)> = Vec::new();
        let dir_path = "data/world/";
        for path in read_dir(dir_path).unwrap() {
            let path_string = String::from(path.unwrap().path().to_str().unwrap());
            let pos = extract_pos_from_path(dir_path.len(), path_string);

            let data = load_chunk(pos).unwrap();
            for (id, world_position) in data.portals() {
               portal_ids.push((*id, *world_position));
            }
        }

        let mut connections: HashMap<Vector2D<i32>, Vector2D<i32>> = HashMap::new();
        for (i_a, id_a) in portal_ids.iter().enumerate() {
            for (i_b, id_b) in portal_ids.iter().enumerate() {
                if i_a == i_b {continue;} 
                if id_a.0 == id_b.0 {
                    connections.insert(id_a.1, id_b.1);
                    break;
                }
            }
        }

        panic!("Success! {:#?} SIZE: {}", connections, connections.len());


        world
    }

    pub fn render(&mut self, renderer: &Renderer, centre_position: Vector2D<i32>) {
        let world_chunk_pos = World::world_to_chunk_position(centre_position);

        for y in -1..=1 {
            for x in -1..=1 {
                let chunk_pos = world_chunk_pos + Vector2D::new(x, y);

                //Load chunks around the center point
                if !self.chunks.contains_key(&chunk_pos) {
                    if let Some(chunk) = Chunk::load(chunk_pos) {
                        self.chunks.insert(chunk_pos, chunk);
                    }
                }

                if let Some(chunk) = self.chunks.get(&chunk_pos) {
                    chunk.render(renderer, centre_position);
                }
            }
        }
    }

    pub fn get_tile(&self, world_position: Vector2D<i32>) -> char {
        let chunk_position = World::world_to_chunk_position(world_position);
        self.chunks.get(&chunk_position).map_or(' ', |chunk| {
            let local_x = world_position.x % CHUNK_SIZE.x;
            let local_y = world_position.y % CHUNK_SIZE.y;
            chunk.get_tile(local_x as usize, local_y as usize)
        })
    }

    fn world_to_chunk_position(world_position: Vector2D<i32>) -> Vector2D<i32> {
        Vector2D::new(
            world_position.x / CHUNK_SIZE.x,
            world_position.y / CHUNK_SIZE.y,
        )
    }

    pub fn is_portal_at(&self, world_position: Vector2D<i32>) -> bool {
        self.get_tile(world_position) == '1'
    }

    pub fn get_portal_at(&self, world_position: Vector2D<i32>) -> Option<&Vector2D<i32>> {
        let chunk_position = World::world_to_chunk_position(world_position);
        let local_x = world_position.x % CHUNK_SIZE.x;
        let local_y = world_position.y % CHUNK_SIZE.y;

        self.chunks
            .get(&chunk_position)
            .unwrap()
            .get_portal(Vector2D::new(local_x, local_y))
    }
}
