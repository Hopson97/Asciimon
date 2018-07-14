mod game_state;
mod player;

use ::graphics::renderer::Renderer;

use self::game_state::GameState;
use self::game_state::state_explore::StateExplore;

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>
}

impl Game {
    pub fn run_game() {
        let mut game = Game {
            renderer: Renderer::new(96, 54),
            state_stack: Vec::new()
        };

        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new()));
        return;
        loop {
            match self.state_stack.last_mut() {
                None => panic!("Game state vector is empty"),
                Some(current_state) => {
                    current_state.input();
                    current_state.update();
                    current_state.draw(&mut self.renderer);
                }
            }
            
        }
    }
}