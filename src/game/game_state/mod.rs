pub mod state_explore;

use ::graphics::renderer::Renderer;

pub trait GameState {
    fn input(&mut self, renderer: &Renderer);
    fn update(&mut self);
    fn draw(&mut self, renderer: &mut Renderer);
}