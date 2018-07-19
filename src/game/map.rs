use ::util::vector::Vector2D;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs;

pub const MAP_WIDTH: i16 = 100;
pub const MAP_HEIGHT: i16 = 80;

pub struct Map {
    world_position: Vector2D<i16>,
    tile_data: Vec<String>,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

impl Map {
    /**
     * Loads a map from a file for coordinates (x, y)
     */
    pub fn load(x: i16, y: i16) -> Option<Map> {
        let mut map = Map {
            world_position: Vector2D::new(x, y),
            tile_data: Vec::with_capacity((MAP_HEIGHT) as usize)
        };

        let mut file_name = String::from("maps/");
        file_name.push_str(x.to_string().as_str());
        file_name.push(' ');
        file_name.push_str(y.to_string().as_str());

        if !path_exists(&file_name) {
            return None
        }
        else {
            let file = File::open(file_name)
                .expect(&format!("Unable to open file for map {} {}", x, y));

            for line in BufReader::new(file).lines() {
                map.tile_data.push(line.unwrap());
                if map.tile_data.len() == MAP_HEIGHT as usize {
                    break;
                }
            }

            Some(map)
        }
    }

    pub fn world_position(&self) -> &Vector2D<i16> {
        &self.world_position
    }

    pub fn data(&self) -> &Vec<String> {
        &self.tile_data
    }
}