use super::map::{Map, MAP_WIDTH, MAP_HEIGHT};
use super::{GAME_AREA_X, GAME_AREA_Y};

use ::util::vector::Vector2D;


use std::collections::HashMap;

pub struct MapManager {
    maps: HashMap<Vector2D<i16>, Map>,
}

impl MapManager {
    pub fn new() -> MapManager {
        MapManager {
            maps: HashMap::new(),
        }
    }

    pub fn update_player_pos(&mut self, player_pos: &Vector2D<i16>) {
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

    pub fn map_lines(&self, player_position: &Vector2D<i16>, center: &Vector2D<u8>) -> Vec<String> {
        let mut map_lines: Vec<String> = vec![String::from(""); GAME_AREA_Y as usize];

        let left_space = (GAME_AREA_X - 1) / 2;
        let top_space  = (GAME_AREA_Y - 1) / 2;

        let world_top_left_x = player_position.x - left_space as i16;
        let world_top_left_y = player_position.y - top_space  as i16;

        let top_y_display = world_top_left_y % MAP_HEIGHT;
        map_lines[0] = (world_top_left_x).to_string();
        map_lines[1] = (world_top_left_x % MAP_WIDTH).to_string();
        




        /*

        let left_space = (GAME_AREA_X - 1) / 2;
        let top_space  = (GAME_AREA_Y - 1) / 2;

        let map_x = player_position.x / MAP_WIDTH;
        let map_y = player_position.y / MAP_HEIGHT;
        let map_offset_x = player_position.x % MAP_WIDTH;
        let map_offset_y = player_position.y % MAP_HEIGHT;

        let map = self.maps.get(&Vector2D::new(map_x, map_y)).unwrap();


        let top_left_world_x = 0;

        for y in map_offset_y..GAME_AREA_Y as i16 {
            map_lines[y as usize] = map.get_line(y as usize).to_string();
        }



        let map = match self.maps.get(&Vector2D::new(map_x, map_y)) {
            None => panic!("A map was not loaded at player position!"),
            Some(map) => map
        };

        for y in 0..MAP_HEIGHT {
            map_lines.push(map.get_line(y as usize).clone());
        }

*/

        map_lines
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