pub mod state_explore;

use ::graphics::renderer::Renderer;

#[allow(dead_code)]
pub enum ReturnResult {
    None,
    StatePush(Box<GameState>),
    StatePop,
    Redraw,

    Exit
}

pub trait GameState {
    fn handle_input(&mut self, input_args: &Vec<&str>) -> ReturnResult;
    fn update(&mut self) -> ReturnResult;
    fn draw(&mut self, renderer: &mut Renderer);
}



