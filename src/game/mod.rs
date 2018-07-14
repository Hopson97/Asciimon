mod game_state;
mod player;
pub mod user_interface;

use ::graphics::renderer::Renderer;
use ::util::vector::Vector2D;

use self::game_state::ReturnResult;
use self::game_state::GameState;
use self::game_state::state_explore::StateExplore;

use self::user_interface as ui;



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
            renderer: Renderer::new(100, 60),
            state_stack: Vec::new(),
            is_running: true
        };
        ui::init(&mut game.renderer);
        game.renderer.add_render_section("game", Vector2D::new(0, 7), Vector2D::new(100, 53));
        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new(&mut self.renderer)));
        
        while self.is_running {
            let input_result: ReturnResult;
            let update_result: ReturnResult;

            match self.state_stack.last_mut() {
                None => panic!("Game state vector is empty"),
                Some(current_state) => {
                    current_state.draw(&mut self.renderer);
                    stdout().flush()
                        .expect("Could not buffer the terminal output!");
                    input_result    = current_state.input(&self.renderer);
                    update_result   = current_state.update();
                }
            } 

            self.handle_return_result(input_result);
            self.handle_return_result(update_result);
        }
    }

    fn handle_return_result(&mut self, input_result: ReturnResult) {
        match input_result {
            ReturnResult::Exit => self.is_running = false,
            ReturnResult::StatePop => {
                self.state_stack.pop();
                if self.state_stack.is_empty() {
                    self.is_running = false;
                }
            },
            ReturnResult::StatePush(state) => {
                self.state_stack.push(state);
            },
            ReturnResult::None => {}
        }
    }
}