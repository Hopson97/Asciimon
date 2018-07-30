use std::collections::HashMap;

use graphics::renderer::Renderer;
use util::vector::Vector2D;

use super::map::{Map, MAP_SIZE};
use super::{GAME_AREA_CENTER, GAME_AREA_SIZE};

pub struct World {
    //error_map: Map,
    maps: HashMap<Vector2D<i32>, Map>,
}

impl World {
    pub fn new() -> World {
        World {
            //error_map: Map::load_from_name(String::from("error").unwrap(),
            maps: HashMap::with_capacity(20),
        }
    }

    pub fn render(&mut self, renderer: &Renderer, player_position: Vector2D<i32>) {
        let map_position = World::player_to_map_position(player_position);
        let x = map_position.x;
        let y = map_position.y;

        for map_y in (y - 1)..(y + 2) {
            for map_x in (x - 1)..(x + 2) {
                let pos = Vector2D::new(map_x, map_y);

                //To do: Improve the cotains key followed by the insert.
                if !self.maps.contains_key(&pos) {
                    if let Some(map) = Map::load(map_x, map_y) {
                        self.maps.insert(pos, map);
                    }
                }
            }
        }

        for map in self.maps.values() {
            World::render_map(&renderer, &map, player_position);
        }
    }

    pub fn get_tile(&self, position: Vector2D<i32>) -> char {
        let map_position = World::player_to_map_position(position);
        let map = match self.maps.get(&map_position) {
            None => panic!(
                "Map at {} {} does not exist!",
                map_position.x, map_position.y
            ),
            Some(map) => map,
        };

        let local_x = position.x % MAP_SIZE.x;
        let local_y = position.y % MAP_SIZE.y;

        map.get_tile(local_x, local_y)
    }

    fn render_map(renderer: &Renderer, map: &Map, player_position: Vector2D<i32>) {
        //Top left position of where the map is drawn from
        let mut map_pos = GAME_AREA_CENTER - player_position + map.world_position * MAP_SIZE;

        //Don't try draw map if it is outside of the bounds of the game rendering area
        if map_pos.x > GAME_AREA_SIZE.x || map_pos.y + MAP_SIZE.x < 0 {
            return;
        }

        //String slice of where the map lines are drawn from and to
        let mut begin_slice = 0;
        let mut end_slice = MAP_SIZE.x - 1;

        if map_pos.x < 0 {
            begin_slice = map_pos.x.abs();
            map_pos.x = 0;
        }

        if map_pos.x + (end_slice - begin_slice) > GAME_AREA_SIZE.x as i32 {
            end_slice = (GAME_AREA_SIZE.x as i32 - map_pos.x) + begin_slice;
        }

        for y in 0..MAP_SIZE.y {
            map.draw_line(
                renderer,
                y as usize,
                begin_slice as usize,
                end_slice as usize,
                map_pos + Vector2D::new(0, y),
            );
        }
    }

    fn player_to_map_position(player_position: Vector2D<i32>) -> Vector2D<i32> {
        player_position / MAP_SIZE
    }
}
