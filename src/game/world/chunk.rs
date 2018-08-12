use graphics::Renderer;

use util::Vector2D;

use game::{GAME_AREA_CENTRE, GAME_AREA_SIZE};

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub const CHUNK_SIZE: Vector2D<i32> = Vector2D { x: 100, y: 50 };

mod colours {
    use graphics::Colour;
    define_colour!(WATER, 32, 178, 230);
    define_colour!(SAND, 232, 210, 99);
    define_colour!(STONE, 200, 200, 200);
    define_colour!(GRASS, 124, 252, 0);
    define_colour!(TALL_GRASS, 10, 130, 10);
    define_colour!(TREE_TRUNK, 160, 82, 45);
    define_colour!(TREE_LEAVES, 34, 100, 34);
    define_colour!(WALL, 200, 200, 205);
    define_colour!(NONE, 255, 0, 255);
}

/// A chunk is a data about a section of the world map.
/// This contains data such as:
/// Tile data
/// Portals (Doors, ladders etc and their destination)
pub struct Chunk {
    world_position: Vector2D<i32>,
    data: Vec<Vec<char>>,
    max_width: usize,
    portals: HashMap<Vector2D<i32>, Vector2D<i32>>, //Local position, then destination
    portal_locations: Vec<Vector2D<i32>>,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

enum MapLoadState {
    FindSecton,
    Map,
    Portals,
}

impl Chunk {
    /**
     * Loads a chunk from a file for coordinates (x, y)
     */
    pub fn load(pos: Vector2D<i32>) -> Option<Chunk> {
        let mut chunk = Chunk {
            world_position: pos,
            data: Vec::with_capacity(CHUNK_SIZE.y as usize),
            max_width: 0,
            portals: HashMap::new(),
            portal_locations: Vec::new(),
        };

        let file_name = format!("data/world/{}_{}.chunk", pos.x, pos.y);

        if !path_exists(&file_name) {
            None //panic!("Path for chunk '{}' does not exist", file_name);
        } else {
            load_chunk(&mut chunk, file_name);
            Some(chunk)
        }
    }

    pub fn render(&self, renderer: &Renderer, centre_position: Vector2D<i32>) {
        //Top left position of where the chunk is drawn from
        let mut chunk_pos = GAME_AREA_CENTRE - centre_position + self.world_position * CHUNK_SIZE;

        // Don't try draw chunk if it is outside of the bounds of the game rendering area
        if chunk_pos.x + CHUNK_SIZE.x <= 0
            || chunk_pos.x >= GAME_AREA_SIZE.x
            || chunk_pos.y + CHUNK_SIZE.y <= 0
            || chunk_pos.y >= GAME_AREA_SIZE.y
        {
            return;
        }

        //String slice of where the chunk lines are drawn from and to
        let mut begin_slice = 0;
        let mut end_slice = self.max_width as i32;

        //Check for the OOB, which prevents the string being drawn before the render section...
        if chunk_pos.x < 0 {
            begin_slice = chunk_pos.x.abs();
            chunk_pos.x = 0;
        }

        //..and after
        if chunk_pos.x + (end_slice - begin_slice) >= GAME_AREA_SIZE.x {
            end_slice = (GAME_AREA_SIZE.x - chunk_pos.x) + begin_slice;
        }

        for y in 0..self.data.len() {
            self.draw_line(
                renderer,
                y as usize,
                begin_slice as usize,
                end_slice as usize,
                chunk_pos + Vector2D::new(0, y as i32),
            );
        }
    }

    ///Draws a single line of the map,
    fn draw_line(
        &self,
        renderer: &Renderer,
        line: usize,
        begin: usize,
        end: usize,
        draw_point: Vector2D<i32>,
    ) {
        let mut render_string = String::with_capacity(CHUNK_SIZE.x as usize * 2);

        // Set colour based on the batch of following chars
        let mut prev_char = ' ';
        for c in &self.data[line][begin..end] {
            if *c != prev_char {
                prev_char = *c;

                let colour = match c {
                    '~' => colours::WATER,
                    '\'' => colours::SAND,
                    'X' => colours::STONE,
                    ',' => colours::TALL_GRASS,
                    '.' => colours::GRASS,
                    '|' => colours::TALL_GRASS,
                    'Y' => colours::TREE_TRUNK,
                    '0' => colours::TREE_LEAVES,
                    '#' => colours::WALL,
                    _ => colours::NONE,
                };
                render_string.push_str(&colour.ansi_text_string());
            }

            render_string.push(*c);
        }

        renderer.draw_string("game", &render_string, draw_point);
    }

    pub fn get_tile(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    pub fn get_portal(&self, local_position: Vector2D<i32>) -> Option<&Vector2D<i32>> {
        println!("GOT PORTAL AT {} ", local_position);
        //  panic!("");

        self.portals.get(&local_position)
    }

    pub fn portal_count(&self) -> usize {
        self.portal_locations.len()
    }

    pub fn portal_locations(&self) -> &Vec<Vector2D<i32>> {
        &self.portal_locations
    }

    pub fn loaded_portals(&self) -> &HashMap<Vector2D<i32>, Vector2D<i32>> {
        &self.portals
    }

    pub fn position(&self) -> Vector2D<i32> {
        self.world_position
    }
}

/// Reads a .chunk file into a Chunk struct
fn load_chunk(chunk: &mut Chunk, file_name: String) {
    let file = File::open(file_name).unwrap_or_else(|_| {
        panic!(
            "Unable to open file for chunk {} {}",
            chunk.world_position.x, chunk.world_position.y
        )
    });

    let mut load_state = MapLoadState::FindSecton;
    //Load the map
    for line in BufReader::new(file).lines() {
        match load_state {
            MapLoadState::FindSecton => {
                match line.unwrap().as_ref() {
                    "map" => load_state = MapLoadState::Map,
                    "portals" => load_state = MapLoadState::Portals,
                    _ => {} //Empty lines
                }
            }
            MapLoadState::Map => {
                let curr_line = line.unwrap();
                match curr_line.as_ref() {
                    "end" => load_state = MapLoadState::FindSecton,
                    _ => {
                        chunk.max_width = curr_line.len();
                        let chars: Vec<char> = curr_line.chars().collect();
                        for (x, tile) in chars.iter().enumerate() {
                            match tile {
                                '1' => {
                                    let y = chunk.data.len();
                                    chunk
                                        .portal_locations
                                        .push(Vector2D::new(x as i32, y as i32));
                                }
                                _ => {}
                            }
                        }
                        chunk.data.push(chars);
                    }
                }
            }
            MapLoadState::Portals => {
                let curr_line = line.unwrap();
                match curr_line.as_ref() {
                    "end" => load_state = MapLoadState::FindSecton,
                    _ => {
                        let portal_data: Vec<&str> = curr_line.trim().split(' ').collect();
                        let mut portal_nums: Vec<i32> = Vec::with_capacity(6);
                        for data in &portal_data {
                            match data.parse::<i32>() {
                                Err(_) => break, //should cancel out the creation of the portal (TODO)
                                Ok(n) => {
                                    portal_nums.push(n);
                                    n
                                }
                            };
                        }

                        let local_portal_location = Vector2D::new(portal_nums[0], portal_nums[1]);
                        let portal_world_dest = Vector2D::new(portal_nums[2], portal_nums[3]);

                        chunk
                            .portals
                            .insert(local_portal_location, portal_world_dest);
                    }
                }
            }
        }
    }
}

//i hate file io ngl
pub fn save_chunk(chunk: &Chunk, connections: &HashMap<Vector2D<i32>, Vector2D<i32>>) {
    let path = format!(
        "data/world/{}_{}.chunk",
        chunk.position().x,
        chunk.position().y
    );
    let mut file = File::create(path).unwrap();
    file.write(b"map\n").expect("Unable to write");

    for line in &chunk.data {
        let mut tiles = String::new();
        for c in line.iter() {
            tiles.push(*c);
        }
        tiles.push('\n');
        file.write(tiles.as_bytes()).expect("Unable to write");
    }
    file.write(b"end\n").expect("Unable to write");
    file.write(b"portals\n").expect("Unable to write");
    for portal in connections.iter() {
        let mut line = format!(
            "{} {} {} {}",
            portal.0.x, portal.0.y, portal.1.x, portal.1.y
        );
        line.push('\n');
        file.write(line.as_bytes()).expect("oops");
    }
    file.write(b"end\n").expect("Unable to write");
}
