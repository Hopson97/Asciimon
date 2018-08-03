use graphics::renderer::Renderer;

use util::vector::Vector2D;

use game::{GAME_AREA_CENTER, GAME_AREA_SIZE};

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs;

pub const CHUNK_SIZE: Vector2D<i32> = Vector2D { x: 100, y: 50 };

mod colours {
    use graphics::colour::Colour;
    define_colour!(WATER, 32, 178, 230);
    define_colour!(SAND, 232, 210, 99);
    define_colour!(STONE, 200, 200, 200);
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

    pub fn render(&self, renderer: &Renderer, player_position: Vector2D<i32>) {
        //Top left position of where the chunk is drawn from
        let mut chunk_pos = GAME_AREA_CENTER - player_position + self.world_position * CHUNK_SIZE;

        // Don't try draw chunk if it is outside of the bounds of the game rendering area
        // (this code may look weird, but it works)
        if chunk_pos.x + CHUNK_SIZE.x <= 0
            || chunk_pos.x >= GAME_AREA_SIZE.x
            || chunk_pos.y + CHUNK_SIZE.y <= 0
            || chunk_pos.y >= GAME_AREA_SIZE.y
        {
            return;
        }

        //String slice of where the chunk lines are drawn from and to
        let mut begin_slice = 0;
        let mut end_slice = CHUNK_SIZE.x;

        if chunk_pos.x < 0 {
            begin_slice = chunk_pos.x.abs();
            chunk_pos.x = 0;
        }

        if chunk_pos.x + (end_slice - begin_slice) >= GAME_AREA_SIZE.x {
            end_slice = (GAME_AREA_SIZE.x - chunk_pos.x) + begin_slice;
        }

        for y in 0..CHUNK_SIZE.y {
            self.draw_line(
                renderer,
                y as usize,
                begin_slice as usize,
                end_slice as usize,
                chunk_pos + Vector2D::new(0, y),
            );
        }
    }

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
                    _ => continue,
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
}
