pub mod state_explore;

use ::graphics::renderer::Renderer;

pub enum ReturnResult {
    None,
    StatePush(Box<GameState>),
    StatePop,
    Redraw,

    Exit
}

pub trait GameState {
    fn input(&mut self, renderer: &Renderer) -> ReturnResult;
    fn update(&mut self) -> ReturnResult;
    fn draw(&mut self, renderer: &mut Renderer);
}