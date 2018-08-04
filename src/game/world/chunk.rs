use graphics::panel::Panel;

use util::vector::Vector2D;

use game::{GAME_AREA_CENTRE, GAME_AREA_SIZE};

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs;

pub const CHUNK_SIZE: Vector2D<u32> = Vector2D { x: 100, y: 50 };

mod colours {
    use graphics::colour::Colour;
    define_colour!(WATER, 32, 178, 230);
    define_colour!(SAND, 232, 210, 99);
    define_colour!(STONE, 200, 200, 200);
    define_colour!(GRASS, 124, 252, 0);
    define_colour!(TALL_GRASS, 10, 130, 10);
    define_colour!(TREE_TRUNK, 160, 82, 45);
    define_colour!(TREE_LEAVES, 34, 100, 34);
}

///Represents a section (of the map) of the world.
/// Contains data about tiles make it up, and what position said tiles are in
pub struct Chunk {
    pub world_position: Vector2D<u32>,
    data: Vec<Vec<char>>,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

impl Chunk {
    /**
     * Loads a chunk from a file for coordinates (x, y)
     */
    pub fn load(pos: Vector2D<u32>) -> Option<Chunk> {
        let mut chunk = Chunk {
            world_position: pos,
            data: Vec::with_capacity(CHUNK_SIZE.y as usize),
        };

        let file_name = format!("data/world/{}_{}.chunk", pos.x, pos.y);

        if !path_exists(&file_name) {
            None //panic!("Path for chunk '{}' does not exist", file_name);
        } else {
            let file = File::open(file_name)
                .unwrap_or_else(|_| panic!("Unable to open file for chunk {} {}", pos.x, pos.y));

            for line in BufReader::new(file).lines() {
                chunk.data.push(line.unwrap().chars().collect());
                if chunk.data.len() == CHUNK_SIZE.y as usize {
                    break;
                }
            }

            Some(chunk)
        }
    }

    pub fn render(&self, panel: &Panel, centre_position: Vector2D<u32>) {
        // Top left position of where the chunk is drawn from
        let chunk_pos = (GAME_AREA_CENTRE + self.world_position * CHUNK_SIZE).to_i32()
            - centre_position.to_i32();

        // Don't try draw chunk if it is outside of the bounds of the game rendering area
        // (this code may look weird, but it works)
        if chunk_pos.x + CHUNK_SIZE.x as i32 <= 0
            || chunk_pos.x >= GAME_AREA_SIZE.x as i32
            || chunk_pos.y + CHUNK_SIZE.y as i32 <= 0
            || chunk_pos.y >= GAME_AREA_SIZE.y as i32
        {
            return;
        }

        // Calculate dimensions and offset of the visible part of the chunk

        let offset: Vector2D<u32> = chunk_pos.map(|n| if n > 0 { n as u32 } else { 0 });
        let slice_start: Vector2D<u32> = chunk_pos.map(|n| if n < 0 { n.abs() as u32 } else { 0 });

        let mut slice_end: Vector2D<u32> = CHUNK_SIZE;
        if offset.x + (CHUNK_SIZE.x - slice_start.x) >= GAME_AREA_SIZE.x {
            slice_end.x = (GAME_AREA_SIZE.x - offset.x) + slice_start.x
        }
        if offset.y + (CHUNK_SIZE.y - slice_start.y) >= GAME_AREA_SIZE.y {
            slice_end.y = (GAME_AREA_SIZE.y - offset.y) + slice_start.y
        }

        for y in slice_start.y..slice_end.y {
            let row = &self.data[y as usize];
            let row_slice = &row[slice_start.x as usize..slice_end.x as usize];

            self.draw_row(
                panel,
                row_slice,
                offset + Vector2D::new(0, y - slice_start.y),
            );
        }
    }

    /// Draws a single row of the map
    fn draw_row(&self, panel: &Panel, line: &[char], draw_point: Vector2D<u32>) {
        let mut render_string = String::with_capacity(CHUNK_SIZE.x as usize * 2);

        // Set colour based on the batch of following chars
        let mut prev_char = ' ';
        for c in line {
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
                    _ => continue,
                };
                render_string.push_str(&colour.ansi_text_string());
            }

            render_string.push(*c);
        }

        panel.draw_string(&render_string, draw_point);
    }

    pub fn get_tile(&self, x: u32, y: u32) -> char {
        self.data[y as usize][x as usize]
    }
}
