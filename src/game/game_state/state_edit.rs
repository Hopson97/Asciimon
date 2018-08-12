use super::GameState;
use game::UpdateResult;

use graphics::Colour;
use graphics::Renderer;

use game::Console;

use util::{Vector2D};

pub struct StateEdit {
    
}

impl StateEdit {
    pub fn new() -> StateEdit {
        StateEdit {
            
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
        None
    }

    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console) {

    }
}