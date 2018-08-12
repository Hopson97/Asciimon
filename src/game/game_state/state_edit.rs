use super::GameState;
use game::UpdateResult;

use graphics::Colour;
use graphics::Renderer;

use game::Console;

use game::world::Chunk;

use util::{Vector2D};

pub struct StateEdit {
    loaded_chunks: Vec<Chunk>,
    view_point: Vector2D<i32>,
    
}

impl StateEdit {
    pub fn new() -> StateEdit {
        StateEdit {
            loaded_chunks: Vec::with_capacity(2),
            view_point: Vector2D::new(0, 0),
        }
    }
}

impl GameState for StateEdit {
    fn write_instructions(&self, console: &mut Console)
    {

    }
    
    fn execute_command(
        &mut self,
        command_args: &[&str],
        console: &mut Console,
    ) -> Option<UpdateResult>
    {
        console.write(&format!("Args: {:?}", command_args));
        match command_args {
            ["load"] => {console.write("loaing and that");},
            ["load", "map", x, y] => {
                console.write(&format!("Loading map at ({}, {})", x, y));
                //TODO: ERROR CHECK
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();

                if let Some(chunk) = Chunk::load(Vector2D::new(x, y)) {
                    self.loaded_chunks.push(chunk);
                } else {
                    console.write(&format!("Map chunk at ({}, {}) does not exist!", x, y));
                }
            },
            _ => {}
        }
        None
    }

    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console) {
        if !self.loaded_chunks.is_empty() {
            let chunk = self.loaded_chunks.last_mut().unwrap();
            chunk.render(renderer, self.view_point);
        }
    }
}