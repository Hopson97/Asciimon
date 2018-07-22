use std::collections::HashMap;

use ::graphics::renderer::Renderer;
use ::util::vector::Vector2D;

use super::map::{Map, MAP_HEIGHT, MAP_WIDTH};
use super::{GAME_AREA_X};
use super::game_state::state_explore::{CENTER_X, CENTER_Y};

pub struct MapManager {
    maps: HashMap<Vector2D<i16>, Map>
} 

impl MapManager {
    pub fn new () -> MapManager {
        MapManager {
            maps: HashMap::with_capacity(20)
        }
    }

    pub fn render_maps(&mut self, renderer: &Renderer, player_position: &Vector2D<i16>) {
        let map_position = MapManager::player_to_map_position(&player_position);
        let x = map_position.x;
        let y = map_position.y;

        if !self.maps.contains_key(&map_position) {
            self.maps.insert(map_position, Map::load(x, y).unwrap());
        }

        //...load more maps duh

        for (_, map) in &self.maps {
            MapManager::draw_map(&renderer, &map, &player_position);
        }
    }

    fn draw_map(renderer: &Renderer, map: &Map, player_position: &Vector2D<i16>) {
        //Top left position of where the map is drawn from
        let mut map_x = CENTER_X as i16 - player_position.x + (MAP_WIDTH  - 1) * map.world_position().x;
        let     map_y = CENTER_Y as i16 - player_position.y + (MAP_HEIGHT - 1) * map.world_position().y;

        //Don't try draw map if it is outside of the bounds of the game rendering area
        if map_x > GAME_AREA_X as i16 || map_x + MAP_WIDTH < 0 {
            return;
        }

        //String slice of where the map lines are drawn from and to
        let mut begin_slice = 0;
        let mut end_slice = (MAP_WIDTH - 1) as i16;

        if map_x < 0 {
            begin_slice = map_x.abs();
            map_x = 0;
        }

        if map_x + (end_slice - begin_slice)  > GAME_AREA_X as i16 {
            end_slice = (GAME_AREA_X as i16 - map_x) + begin_slice;
        }

        for y in 0..MAP_HEIGHT {
            map.draw_line(
                renderer, 
                y           as usize, 
                begin_slice as usize, 
                end_slice   as usize, 
                &Vector2D::new(map_x, map_y + y));
        }
    }

    fn player_to_map_position(player_position: &Vector2D<i16>) -> Vector2D<i16> {
        Vector2D::new(player_position.x / MAP_WIDTH, player_position.y / MAP_HEIGHT)
    }
}