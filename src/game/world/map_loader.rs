use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use util::Vector2D;
use super::chunk::CHUNK_SIZE;

enum MapLoadState {
    FindSecton,
    Map,
    Portals,
}

pub struct MapLoadData {
    pub tile_data: Vec<Vec<char>>,
    pub portal_positions: Vec<Vector2D<i32>>,
    pub portal_ids:       Vec<i32>
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn map_file_name(location: Vector2D<i32>) -> String {
    format!("data/world/{}_{}.chunk", location.x, location.y)
}

fn load_section_line(line: String, data: &mut MapLoadData, line_handler: fn(String, &mut MapLoadData)) -> bool {
    match line.as_ref() {
        "end" => false,
        _ => {
            line_handler(line, data);
            true
        }
    }
}

fn handle_map_line(line: String, data: &mut MapLoadData) {
    let char_line: Vec<char> = line.chars().collect();

    for (x, ch) in char_line.iter().enumerate() {
        match ch {
            '1' => {
                let y = data.tile_data.len() as i32;
                data.portal_positions.push(Vector2D::new(x as i32, y))
            },
            _ => {} 
        }
    }
    data.tile_data.push(char_line);
}

fn handle_portal_line(line: String, data: &mut MapLoadData) {
    let id = line.parse::<i32>();
    match id {
        Ok(id) => data.portal_ids.push(id),
        Err(_) => panic!("Map ID must be a integral value!")
    }
}

pub fn load_map(location: Vector2D<i32>) -> Option<MapLoadData> {
    let mut data = MapLoadData {
        tile_data: Vec::with_capacity(CHUNK_SIZE.y as usize),
        portal_positions:   Vec::with_capacity(5),
        portal_ids:         Vec::with_capacity(5),
    };

    let path = map_file_name(location);
    if !path_exists(&path) {
        return None
    }

    let file = File::open(path).unwrap(); 
    let mut load_state = MapLoadState::FindSecton;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        match load_state {
            MapLoadState::FindSecton => {
                match line.as_ref() {
                    "map" => load_state = MapLoadState::Map,
                    "portals" => load_state = MapLoadState::Portals,
                    _ => {}
                }
            }
            MapLoadState::Map => {
                if !load_section_line(line, &mut data, handle_map_line) {
                    load_state = MapLoadState::FindSecton;
                }
            },
            MapLoadState::Portals => {
                if !load_section_line(line, &mut data, handle_portal_line) {
                    load_state = MapLoadState::FindSecton;
                }
            }
        }
    }


    Some(data)
}
