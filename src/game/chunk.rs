use graphics::colour::Colour;
use graphics::renderer::Renderer;

use util::vector::Vector2D;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs;

pub const CHUNK_SIZE: Vector2D<i32> = Vector2D { x: 100, y: 50 };

pub struct Chunk {
    pub world_position: Vector2D<i32>,
    tile_data: Vec<String>,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

impl Chunk {
    /**
     * Loads a chunk from a file for coordinates (x, y)
     */
    pub fn load(x: i32, y: i32) -> Option<Chunk> {
        let mut chunk = Chunk {
            world_position: Vector2D::new(x, y),
            tile_data: Vec::with_capacity(CHUNK_SIZE.y as usize),
        };

        let mut file_name = String::from("world/");
        file_name.push_str(x.to_string().as_str());
        file_name.push(' ');
        file_name.push_str(y.to_string().as_str());

        if !path_exists(&file_name) {
            None //panic!("Path for chunk '{}' does not exist", file_name);
        } else {
            let file = File::open(file_name)
                .unwrap_or_else(|_| panic!("Unable to open file for chunk {} {}", x, y));

            for line in BufReader::new(file).lines() {
                chunk.tile_data.push(line.unwrap());
                if chunk.tile_data.len() == CHUNK_SIZE.y as usize {
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
        let ref_string = &self.tile_data[line];

        //Set colour based on the batch of following chars
        //Rust doesn't have static/global objects as far as I know, so I have to implement using a match
        let mut cur_char = ' ';
        for c in ref_string[begin..end].chars() {
            if c != cur_char {
                cur_char = c;
                match c {
                    ',' => render_string.push_str(&Colour::new(14, 160, 20).ansi_text_string()),
                    '|' => render_string.push_str(&Colour::new(30, 145, 35).ansi_text_string()),
                    '.' => render_string.push_str(&Colour::new(124, 252, 0).ansi_text_string()),
                    '#' => render_string.push_str(&Colour::new(34, 100, 34).ansi_text_string()),
                    '0' => render_string.push_str(&Colour::new(34, 100, 34).ansi_text_string()),
                    'Y' => render_string.push_str(&Colour::new(160, 82, 45).ansi_text_string()),
                    '~' => render_string.push_str(&Colour::new(32, 178, 230).ansi_text_string()),
                    'X' => render_string.push_str(&Colour::new(200, 200, 200).ansi_text_string()),
                    _ => {}
                }
            }
            render_string.push(c);
        }

        renderer.draw_string("game", &render_string, draw_point);
    }

    pub fn get_tile(&self, x: i32, y: i32) -> char {
        let line = &self.tile_data[y as usize];

        line.as_bytes()[x as usize] as char
    }
}
