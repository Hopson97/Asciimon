mod game_state;

use ::graphics::renderer::Renderer;

use self::game_state::GameState;

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>
}

impl Game {
    pub fn run_game() {
        let game = Game {
            renderer: Renderer::new(90, 60),
            state_stack: Vec::new()
        };
    }
}