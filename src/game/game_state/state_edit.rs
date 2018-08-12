use super::GameState;
use game::UpdateResult;

use graphics::Colour;
use graphics::Renderer;

use game::Console;

use game::world::Chunk;

use util::{Vector2D};

//To do: When adding a new edit mode, probably best if I split out the different modes to their own implementation

enum EditMode {
    None,
    Map,
    Portal,
}

pub struct StateEdit {
    loaded_chunks: Vec<Chunk>,
    view_point: Vector2D<i32>,
    edit_mode: EditMode,
    portal_ptr: usize,
}

impl StateEdit {
    pub fn new() -> StateEdit {
        StateEdit {
            loaded_chunks: Vec::with_capacity(2),
            view_point: Vector2D::new(0, 0),
            edit_mode: EditMode::None,
            portal_ptr: 0,
        }
    }

    fn curr_chunk(&mut self) -> &Chunk {
        self.loaded_chunks.last_mut().unwrap()
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
        match self.edit_mode {
            EditMode::None => {
                match command_args {
                    ["load"] => {console.write("loaing and that");},
                    ["load", "map", x, y] => {
                        //TODO: ERROR CHECK
                        let x = x.parse::<i32>().unwrap();
                        let y = y.parse::<i32>().unwrap();
                        if let Some(chunk) = Chunk::load(Vector2D::new(x, y)) {
                            self.loaded_chunks.push(chunk);
                            self.edit_mode = EditMode::Map;
                            console.write_with_colour(&format!("Loading map at ({}, {})", x, y), Colour::new(255, 0, 0));
                        } else {
                            console.write_with_colour(&format!("Map chunk at ({}, {}) does not exist!", x, y), Colour::new(255, 0, 0));
                        }
                    },
                    ["back"] => {
                        return Some(UpdateResult::StatePop);
                    }
                    _ => {}
                }
            },
            EditMode::Map => {
                match command_args { 
                    ["portal"] => {
                        if self.curr_chunk().portal_count() > 0 {
                            self.edit_mode = EditMode::Portal;
                        } else {
                            console.write_with_colour(&format!("This map chunk contains no portals"), Colour::new(255, 0, 0));
                        }
                    }
                    ["back"] => {
                        self.edit_mode = EditMode::None;
                    }
                    _ => {}
                }
            },
            EditMode::Portal => {
                match command_args { 
                    ["next"] => {
                        self.portal_ptr += 1;
                        //TODO probably use the % operator
                        if self.curr_chunk().portal_count() == self.portal_ptr {
                            self.portal_ptr = 0;
                        }
                    },
                    ["back"] => {
                        self.edit_mode = EditMode::Map;
                    }
                    _ => {}
                }
            },
        }
    

        None
    }

    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console) {
        if !self.loaded_chunks.is_empty() {
            let chunk = self.loaded_chunks.last_mut().unwrap();

            match self.edit_mode {
                EditMode::Map => {
                    chunk.render(renderer, self.view_point);
                },
                EditMode::Portal => {
                    let position = chunk.portal_locations()[self.portal_ptr];
                    chunk.render(renderer, *position);
                }
                _ => {}
            }
        }
    }
}