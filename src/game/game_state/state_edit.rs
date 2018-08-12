///Unused for now what a messy file
use super::GameState;
use game::UpdateResult;

use graphics::Colour;
use graphics::Renderer;

use game::Console;

use game::world::{chunk::save_chunk, Chunk, CHUNK_SIZE};

use util::Vector2D;

use std::collections::HashMap;

//To do: When adding a new edit mode, probably best if I split out the different modes to their own implementation
#[derive(Debug)]
enum EditMode {
    None,
    Map,
    Portal,
    PortalConnect,
}

struct LoadedChunk {
    chunk: Chunk,
    portal_connections: HashMap<Vector2D<i32>, Vector2D<i32>>,
}

impl LoadedChunk {
    pub fn new(position: Vector2D<i32>) -> Option<LoadedChunk> {
        if let Some(chunk) = Chunk::load(position) {
            let lc = LoadedChunk {
                portal_connections: chunk.loaded_portals().clone(),
                chunk: chunk,
            };
            Some(lc)
        } else {
            None
        }
    }
}

pub struct StateEdit {
    loaded_chunks: Vec<LoadedChunk>,
    view_point: Vector2D<i32>,
    edit_mode: EditMode,
    portal_ptr: usize,
    connect_from: Vector2D<i32>,
    connect_from_world: Vector2D<i32>,
}

impl StateEdit {
    pub fn new() -> StateEdit {
        StateEdit {
            loaded_chunks: Vec::with_capacity(2),
            view_point: Vector2D::new(0, 0),
            edit_mode: EditMode::None,
            portal_ptr: 0,
            connect_from: Vector2D::new(0, 0),
            connect_from_world: Vector2D::new(0, 0),
        }
    }

    fn curr_chunk(&mut self) -> &LoadedChunk {
        self.loaded_chunks.last_mut().unwrap()
    }

    fn try_load_chunk(&mut self, x: &str, y: &str) -> bool {
        //to do errors and that
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        if let Some(chunk) = LoadedChunk::new(Vector2D::new(x, y)) {
            self.loaded_chunks.push(chunk);
            self.edit_mode = EditMode::Map;
            true
        } else {
            false
        }
    }
}

impl GameState for StateEdit {
    fn write_instructions(&self, console: &mut Console) {}

    fn execute_command(
        &mut self,
        command_args: &[&str],
        console: &mut Console,
    ) -> Option<UpdateResult> {
        match self.edit_mode {
            EditMode::None => match command_args {
                ["load", "map", x, y] => {
                    if self.try_load_chunk(x, y) {
                        console.write_with_colour(
                            &format!("Loaded map at ({}, {})", x, y),
                            Colour::new(0, 255, 0),
                        );
                    } else {
                        console.write_with_colour(
                            &format!("Map chunk at ({}, {}) does not exist!", x, y),
                            Colour::new(255, 0, 0),
                        );
                    }
                }
                ["back"] => {
                    return Some(UpdateResult::StatePop);
                }
                _ => {}
            },
            EditMode::Map => match command_args {
                ["portal"] => {
                    if self.curr_chunk().chunk.portal_count() > 0 {
                        self.edit_mode = EditMode::Portal;
                    } else {
                        console.write_with_colour(
                            &format!("This map chunk contains no portals"),
                            Colour::new(255, 0, 0),
                        );
                    }
                }
                ["back"] => {
                    self.edit_mode = EditMode::None;
                }
                _ => {}
            },
            EditMode::Portal => {
                match command_args {
                    ["next"] => {
                        self.portal_ptr += 1;
                        //TODO probably use the % operator?
                        if self.curr_chunk().chunk.portal_count() == self.portal_ptr {
                            self.portal_ptr = 0;
                        }
                    }
                    ["set", "dest", x, y] => {
                        let ptr = self.portal_ptr;
                        self.connect_from = self.curr_chunk().chunk.portal_locations()[ptr];
                        self.connect_from_world = add_chunk_position(
                            self.connect_from,
                            self.curr_chunk().chunk.position(),
                        );
                        if self.try_load_chunk(x, y) {
                            if self.curr_chunk().chunk.portal_count() > 0 {
                                console.write_with_colour(
                                    "Please type 'select' to select the portal to connect to.",
                                    Colour::new(0, 255, 0),
                                );
                                console.write_with_colour(
                                    "Please type 'next' to find the portal to connect to.",
                                    Colour::new(0, 255, 0),
                                );
                                self.portal_ptr = 0;
                                self.edit_mode = EditMode::PortalConnect;
                            } else {
                                self.loaded_chunks.pop();
                                console.write_with_colour(
                                    &format!("This map chunk contains no portals"),
                                    Colour::new(255, 0, 0),
                                );
                            }
                        } else {
                            console.write_with_colour(
                                &format!("Map chunk at ({}, {}) does not exist!", x, y),
                                Colour::new(255, 0, 0),
                            );
                        }
                    }
                    ["back"] => {
                        self.edit_mode = EditMode::Map;
                        console.write_with_colour("Returning to map mode", Colour::new(255, 0, 0));
                    }
                    _ => {}
                }
            }
            EditMode::PortalConnect => {
                match command_args {
                    ["next"] => {
                        self.portal_ptr += 1;
                        //TODO probably use the % operator?
                        if self.curr_chunk().chunk.portal_count() == self.portal_ptr {
                            self.portal_ptr = 0;
                        }
                    }
                    ["select"] => {
                        let mut to_destination: Vector2D<i32>;
                        {
                            let chunk = self.loaded_chunks.last_mut().unwrap();
                            let local = chunk.chunk.portal_locations()[self.portal_ptr];
                            to_destination = add_chunk_position(local, chunk.chunk.position());

                            chunk
                                .portal_connections
                                .insert(local, self.connect_from_world);
                            save_chunk(&chunk.chunk, &chunk.portal_connections);
                        }
                        self.loaded_chunks.pop();
                        let chunk = self.loaded_chunks.last_mut().unwrap();
                        chunk
                            .portal_connections
                            .insert(self.connect_from, to_destination);

                        save_chunk(&chunk.chunk, &chunk.portal_connections);
                        self.portal_ptr = 0;
                        self.edit_mode = EditMode::Map;
                    }
                    ["back"] => {
                        self.edit_mode = EditMode::Portal;
                        self.loaded_chunks.pop();
                        console
                            .write_with_colour("Returning to portal mode", Colour::new(255, 0, 0));
                    }
                    _ => {}
                }
            }
        }

        None
    }

    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console) {
        console.write(&format!("Current editing mode: {:?}.", self.edit_mode));
        if !self.loaded_chunks.is_empty() {
            let chunk = self.loaded_chunks.last_mut().unwrap();

            match self.edit_mode {
                EditMode::Map => {
                    chunk.chunk.render(renderer, Vector2D::new(0, 0));
                }
                EditMode::Portal | EditMode::PortalConnect => {
                    let position = chunk.chunk.portal_locations()[self.portal_ptr];
                    let world_position = add_chunk_position(position, chunk.chunk.position());
                    chunk.chunk.render(renderer, world_position);
                }
                _ => {}
            }
        }
    }
}

fn add_chunk_position(local: Vector2D<i32>, world: Vector2D<i32>) -> Vector2D<i32> {
    local + Vector2D::new(world.x * CHUNK_SIZE.x, world.y * CHUNK_SIZE.y)
}
