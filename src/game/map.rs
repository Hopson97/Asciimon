use graphics::colour::Colour;
use graphics::renderer::Renderer;

use util::vector::Vector2D;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs;

pub const MAP_SIZE: Vector2D<i32> = Vector2D { x: 100, y: 50 };

pub struct Map {
    world_position: Vector2D<i32>,
    tile_data: Vec<String>,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

impl Map {
    /**
     * Loads a map from a file for coordinates (x, y)
     */
    pub fn load(x: i32, y: i32) -> Option<Map> {
        let mut map = Map {
            world_position: Vector2D::new(x, y),
            tile_data: Vec::with_capacity(MAP_SIZE.y as usize),
        };

        let mut file_name = String::from("maps/");
        file_name.push_str(x.to_string().as_str());
        file_name.push(' ');
        file_name.push_str(y.to_string().as_str());

        if !path_exists(&file_name) {
            None //panic!("Path for map '{}' does not exist", file_name);
        } else {
            let file = File::open(file_name)
                .unwrap_or_else(|_| panic!("Unable to open file for map {} {}", x, y));

            for line in BufReader::new(file).lines() {
                map.tile_data.push(line.unwrap());
                if map.tile_data.len() == MAP_SIZE.y as usize {
                    break;
                }
            }

            Some(map)
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
        let mut render_string = String::with_capacity(MAP_SIZE.x as usize * 2);
        let ref_string = &self.tile_data[line];

        //Set colour based on the batch of following chars
        //Rust doesn't have static/global objects as far as I know, so I have to implement using a match
        let mut cur_char = ' ';
        for c in ref_string[begin..end].chars() {
            if c != cur_char {
                cur_char = c;
                match c {
                    ',' => render_string.push_str(&Colour::ansi_text_colour_string(14, 160, 20)),
                    '|' => render_string.push_str(&Colour::ansi_text_colour_string(30, 145, 35)),
                    '.' => render_string.push_str(&Colour::ansi_text_colour_string(124, 252, 0)),
                    '#' => render_string.push_str(&Colour::ansi_text_colour_string(34, 100, 34)),
                    '0' => render_string.push_str(&Colour::ansi_text_colour_string(34, 100, 34)),
                    'Y' => render_string.push_str(&Colour::ansi_text_colour_string(160, 82, 45)),
                    '~' => render_string.push_str(&Colour::ansi_text_colour_string(32, 178, 230)),
                    'X' => render_string.push_str(&Colour::ansi_text_colour_string(200, 200, 200)),
                    _ => {}
                }
            }
            render_string.push(c);
        }

        renderer.draw_string("game", &render_string, &draw_point);
    }

    pub fn get_tile(&self, x: i32, y: i32) -> char {
        let line = &self.tile_data[y as usize];

        line.as_bytes()[x as usize] as char
    }

    pub fn world_position(&self) -> &Vector2D<i32> {
        &self.world_position
    }
}
