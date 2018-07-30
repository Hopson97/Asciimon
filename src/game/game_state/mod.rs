pub mod state_explore;

use graphics::renderer::Renderer;

#[allow(dead_code)]
pub enum ReturnResult {
    None,
    StatePush(Box<GameState>),
    StatePop,
    Redraw,

    Exit,
}

pub trait GameState {
    fn update(&mut self, input_args: &[&str]) -> ReturnResult;
    fn draw(&mut self, renderer: &mut Renderer);
}
