pub mod state_explore;

use game::console::Console;
use game::UpdateResult;
use graphics::renderer::Renderer;

pub trait GameState {
    fn write_instructions(&self, console: &mut Console);
    fn execute_command(
        &mut self,
        command_args: &[&str],
        console: &mut Console,
    ) -> Option<UpdateResult>;
    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console);
}
