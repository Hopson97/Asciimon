pub mod state_explore;

use game::console::Console;
use game::UpdateResult;
use graphics::renderer::Renderer;

pub trait GameState {
    fn write_instructions(&self, console: &mut Console);
    fn tick(&mut self, input_args: &[&str], console: &mut Console) -> Option<UpdateResult>;
    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console);
}
