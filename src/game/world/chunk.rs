use graphics::Renderer;

use util::Vector2D;

use game::{GAME_AREA_CENTRE, GAME_AREA_SIZE};

use std::collections::HashMap;

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
    tile_data: Vec<Vec<char>>,
    max_width: usize,
}

impl Chunk {
    pub fn new(pos: Vector2D<i32>, tile_data: Vec<Vec<char>>) -> Chunk {
        let mut chunk = Chunk {
            world_position: pos,
            tile_data: tile_data,
            max_width: 0,
        };

        chunk.max_width = chunk.tile_data[0].len();

        chunk
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

        for y in 0..self.tile_data.len() {
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
        for c in &self.tile_data[line][begin..end] {
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
        self.tile_data[y][x]
    }
}