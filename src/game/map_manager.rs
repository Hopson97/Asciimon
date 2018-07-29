use std::collections::HashMap;

use graphics::renderer::Renderer;
use util::vector::Vector2D;

use super::game_state::state_explore::{CENTER_X, CENTER_Y};
use super::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use super::GAME_AREA_X;

pub struct MapManager {
    //error_map: Map,
    maps: HashMap<Vector2D<i32>, Map>,
}

impl MapManager {
    pub fn new() -> MapManager {
        MapManager {
            //error_map: Map::load_from_name(String::from("error").unwrap(),
            maps: HashMap::with_capacity(20),
        }
    }

    pub fn render_maps(&mut self, renderer: &Renderer, player_position: &Vector2D<i32>) {
        let map_position = MapManager::player_to_map_position(&player_position);
        let x = map_position.x;
        let y = map_position.y;

        for map_y in (y - 1)..(y + 2) {
            for map_x in (x - 1)..(x + 2) {
                let pos = Vector2D::new(map_x, map_y);

                //To do: Improve the cotains key followed by the insert.
                if !self.maps.contains_key(&pos) {
                    let map = match Map::load(map_x, map_y) {
                        None => continue,
                        Some(map) => map,
                    };

                    self.maps.insert(pos, map);
                }
            }
        }

        for map in self.maps.values() {
            MapManager::draw_map(&renderer, &map, &player_position);
        }
    }

    pub fn get_tile(&self, position: &Vector2D<i32>) -> char {
        let map_position = MapManager::player_to_map_position(&position);
        let map = match self.maps.get(&map_position) {
            None => panic!(
                "Map at {} {} does not exist!",
                map_position.x, map_position.y
            ),
            Some(map) => map,
        };

        let local_x = position.x % MAP_WIDTH;
        let local_y = position.y % MAP_HEIGHT;

        map.get_tile(local_x, local_y)
    }

    fn draw_map(renderer: &Renderer, map: &Map, player_position: &Vector2D<i32>) {
        //Top left position of where the map is drawn from
        let mut map_x = CENTER_X - player_position.x + (MAP_WIDTH) * map.world_position().x;
        let map_y = CENTER_Y - player_position.y + (MAP_HEIGHT) * map.world_position().y;

        //Don't try draw map if it is outside of the bounds of the game rendering area
        if map_x > GAME_AREA_X || map_x + MAP_WIDTH < 0 {
            return;
        }

        //String slice of where the map lines are drawn from and to
        let mut begin_slice = 0;
        let mut end_slice = MAP_WIDTH - 1;

        if map_x < 0 {
            begin_slice = map_x.abs();
            map_x = 0;
        }

        if map_x + (end_slice - begin_slice) > GAME_AREA_X as i32 {
            end_slice = (GAME_AREA_X as i32 - map_x) + begin_slice;
        }

        for y in 0..MAP_HEIGHT {
            map.draw_line(
                renderer,
                y as usize,
                begin_slice as usize,
                end_slice as usize,
                &Vector2D::new(map_x, map_y + y),
            );
        }
    }

    fn player_to_map_position(player_position: &Vector2D<i32>) -> Vector2D<i32> {
        Vector2D::new(
            player_position.x / MAP_WIDTH,
            player_position.y / MAP_HEIGHT,
        )
    }
}
