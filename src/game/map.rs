use ::util::vector::Vector2D;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub struct Map {
    coordinate: Vector2D<i16>,
    tile_data: Vec<String>
}

impl Map {
    /**
     * Loads a map from a file for coordinates (x, y)
     */
    pub fn load(x: i16, y: i16) -> Map {
        let mut map = Map {
            coordinate: Vector2D::new(x, y),
            tile_data: Vec::with_capacity(32)
        };

        let mut file_name = String::from("maps/");
        file_name.push_str(x.to_string().as_str());
        file_name.push(' ');
        file_name.push_str(y.to_string().as_str());

        let file = File::open(file_name)
            .expect(&format!("Unable to open file for map {} {}", x, y));

        //Each map should be 64x32
        for line in BufReader::new(file).lines() {
            map.tile_data.push(line.unwrap());
            if map.tile_data.len() == 32 {
                break;
            }
        }

        map
    }
}