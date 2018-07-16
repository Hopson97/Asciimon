use super::map::{Map, MAP_WIDTH, MAP_HEIGHT};
use ::util::vector::Vector2D;

use std::collections::HashMap;

pub struct MapManager {
    maps: HashMap<Vector2D<i16>, Map>,
    player_current_pos: Vector2D<i16>
}

impl MapManager {
    pub fn new() -> MapManager {
        MapManager {
            maps: HashMap::new(),
            player_current_pos: Vector2D::new(0, 0)
        }
    }

    pub fn update_player_pos(&mut self, player_pos: &Vector2D<i16>) {
        self.player_current_pos = player_pos.clone();
        let min_map_x = player_pos.x / MAP_WIDTH - 2;
        let min_map_y = player_pos.y / MAP_HEIGHT - 2;

        let max_map_x = min_map_x + 4;
        let max_map_y = min_map_y + 4;

        for y in min_map_y..max_map_y {
            for x in min_map_x..max_map_x {
                self.try_load_map(x, y);
            }
        }

        self.maps.retain(|pos, _| {
            !(pos.x < min_map_x || pos.x > max_map_x || 
               pos.y < min_map_y || pos.y > max_map_y)
        });
    }

    fn try_load_map(&mut self, x: i16, y: i16) {
        let map_coords = Vector2D::new(x, y);
        if !self.maps.contains_key(&map_coords) {
            match Map::load(x, y) {
                None => {},
                Some(map) => {
                    self.maps.insert(map_coords.clone(), map);
                }
            }
        }
    }
        


}