pub mod state_explore;
pub mod state_edit;

pub use self::state_explore::StateExplore;
pub use self::state_edit::StateEdit;

use game::Console;
use game::UpdateResult;
use graphics::Renderer;

pub trait GameState {
    fn write_instructions(&self, console: &mut Console);
    
    fn execute_command(
        &mut self,
        command_args: &[&str],
        console: &mut Console,
    ) -> Option<UpdateResult>;

    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console);
}
