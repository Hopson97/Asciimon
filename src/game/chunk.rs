use graphics::renderer::Renderer;

use util::vector::Vector2D;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs;

pub const CHUNK_SIZE: Vector2D<i32> = Vector2D { x: 100, y: 50 };

mod colours {
    use graphics::colour::Colour;
    define_colour!(WATER, 32, 178, 230);
    define_colour!(STONE, 200, 200, 200);
    define_colour!(BUSH, 14, 160, 20);
    define_colour!(GRASS, 124, 252, 0);
    define_colour!(TALL_GRASS, 30, 145, 35);
    define_colour!(TREE_TRUNK, 160, 82, 45);
    define_colour!(TREE_LEAVES, 34, 100, 34);
}

pub struct Chunk {
    pub world_position: Vector2D<i32>,
    data: Vec<Vec<char>>,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

impl Chunk {
    /**
     * Loads a chunk from a file for coordinates (x, y)
     */
    pub fn load(pos: Vector2D<i32>) -> Option<Chunk> {
        let mut chunk = Chunk {
            world_position: pos,
            data: Vec::with_capacity(CHUNK_SIZE.y as usize),
        };

        let file_name = format!("world/{}_{}.chunk", pos.x, pos.y);

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

    pub fn draw_line(
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
        for c in &self.data[line][begin..=end] {
            if *c != prev_char {
                prev_char = *c;

                let colour = match c {
                    '~' => colours::WATER,
                    'X' => colours::STONE,
                    ',' => colours::BUSH,
                    '.' => colours::GRASS,
                    '|' => colours::TALL_GRASS,
                    'Y' => colours::TREE_TRUNK,
                    '#' => colours::TREE_LEAVES,
                    '0' => colours::TREE_LEAVES,
                    _ => continue,
                };
                render_string.push_str(&colour.ansi_text_string());
            }

            render_string.push(*c);
        }

        renderer.draw_string("game", &render_string, draw_point);
    }

    pub fn get_tile(&self, x: i32, y: i32) -> char {
        self.data[y as usize][x as usize]
    }
}
