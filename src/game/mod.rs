mod game_state;
mod player;
pub mod user_interface;

use ::graphics::renderer::Renderer;


use self::game_state::GameState;
use self::game_state::state_explore::StateExplore;

use std::io::Write;
use std::io::stdout;

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>,
    is_running: bool
}

impl Game {
    pub fn run_game() {
        let mut game = Game {
            renderer: Renderer::new(96, 54),
            state_stack: Vec::new(),
            is_running: true
        };
        
        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new(&mut self.renderer)));
        
        while self.is_running {
            match self.state_stack.last_mut() {
                None => panic!("Game state vector is empty"),
                Some(current_state) => {
                    stdout().flush()
                        .expect("Could not buffer the terminal output!");
                    current_state.input();
                    current_state.update();
                    current_state.draw(&mut self.renderer);
                }
            } 
            break;
        }
    }
}