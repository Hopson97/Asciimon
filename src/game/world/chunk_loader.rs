use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use util::Vector2D;
use super::chunk::CHUNK_SIZE;

/// Used to identify the different sections of the .chunk files
enum ChunkLoadState {
    FindSecton,
    Map,
    Portals,
}

pub struct ChunkLoadData {
    tile_data: Vec<Vec<char>>,
    portal_positions: Vec<Vector2D<i32>>,
    portal_ids: Vec<i32>,
    portals: HashMap<i32, Vector2D<i32>>
}

impl ChunkLoadData {
    fn new() -> ChunkLoadData {
        ChunkLoadData {
            tile_data: Vec::with_capacity(CHUNK_SIZE.y as usize),
            portal_positions:   Vec::with_capacity(5),
            portal_ids:         Vec::with_capacity(5),
            portals:            HashMap::with_capacity(5)
        }
    }

    pub fn tile_data(&self) -> &Vec<Vec<char>> {
        &self.tile_data
    }

    pub fn portals(&self) -> &HashMap<i32, Vector2D<i32>> {
        &self.portals
    }
}

/// Extracts X/Y from a chunk file
/// Eg, '2_5.chunk' as the path_string param would return x: 2, y: 5
pub fn extract_pos_from_path(dir_path_len: usize, path_string: String) -> Vector2D<i32> {
    let x_end = path_string.find('_').unwrap();
    let y_end = path_string.find('.').unwrap();
    let x = &path_string[dir_path_len..x_end ];
    let y = &path_string[x_end + 1..y_end];
                
    let x = x.parse::<i32>().unwrap();
    let y = y.parse::<i32>().unwrap();

    Vector2D::new(x, y)
}

///Reads a chunk file and extracts the data from it
pub fn load_chunk(location: Vector2D<i32>) -> Option<ChunkLoadData> {
    let mut data = ChunkLoadData::new();

    let path = map_file_name(location);
    if !path_exists(&path) {
        return None
    }

    let file = File::open(path).unwrap(); 
    let mut load_state = ChunkLoadState::FindSecton;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        match load_state {
            ChunkLoadState::FindSecton => {
                match line.as_ref() {
                    "map" => load_state = ChunkLoadState::Map,
                    "portals" => load_state = ChunkLoadState::Portals,
                    _ => {}
                }
            }
            ChunkLoadState::Map => {
                if !load_section_line(line, &mut data, handle_map_line) {
                    load_state = ChunkLoadState::FindSecton;
                }
            },
            ChunkLoadState::Portals => {
                if !load_section_line(line, &mut data, handle_portal_line) {
                    load_state = ChunkLoadState::FindSecton;
                }
            }
        }
    }

    //Check all portals have a matching ID
    if !(data.portal_ids.len() == data.portal_positions.len()) {
        return None
    }
    
    //Add found portals into the map
    for (i, portal_id) in data.portal_ids.iter().enumerate() {
        data.portals.insert(
            *portal_id, 
            add_chunk_position(data.portal_positions[i], location)
        );
    }

    Some(data)
}

/// on tin
fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/// Gets map file from from x/y coords
fn map_file_name(location: Vector2D<i32>) -> String {
    format!("data/world/{}_{}.chunk", location.x, location.y)
}

/// Just so that I don't have to repeat writing match with "end" over and over
fn load_section_line(line: String, data: &mut ChunkLoadData, line_handler: fn(String, &mut ChunkLoadData)) -> bool {
    match line.as_ref() {
        "end" => false,
        _ => {
            line_handler(line, data);
            true
        }
    }
}

/// Reads and extracts a line from the map section of the .chunk file
fn handle_map_line(line: String, data: &mut ChunkLoadData) {
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

/// Reads and extracts a line from the portal section of the .chunk file
fn handle_portal_line(line: String, data: &mut ChunkLoadData) {
    let id = line.parse::<i32>();
    match id {
        Ok(id) => data.portal_ids.push(id),
        Err(_) => panic!("Map ID must be a integral value!")
    }
}

///Adds the world position of a chunk to a local world position to that chunk
fn add_chunk_position(local: Vector2D<i32>, world: Vector2D<i32>) -> Vector2D<i32> {
    local + Vector2D::new(world.x * CHUNK_SIZE.x, world.y * CHUNK_SIZE.y)
}