mod game_state;
mod player;

pub mod map;
pub mod user_interface;
pub mod map_manager;

use ::graphics::colour::Colour;
use ::graphics::renderer::Renderer;
use ::util::vector::Vector2D;

use self::game_state::ReturnResult;
use self::game_state::GameState;
use self::game_state::state_explore::StateExplore;

use self::user_interface as ui;

use std::io::Write;
use std::io::stdout;

pub const GAME_AREA_X: u8 = 81;
pub const GAME_AREA_Y: u8 = 45;

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>,
    is_running: bool,
    needs_redraw: bool
}

impl Game {
    pub fn run_game() {
        let mut game = Game {
            renderer: Renderer::new(81, 52),
            state_stack: Vec::new(),
            is_running: true,
            needs_redraw: true
        };
        ui::init(&mut game.renderer);
        game.renderer.add_render_section("game", Vector2D::new(0, 7), Vector2D::new(GAME_AREA_X, GAME_AREA_Y));
        game.renderer.clear_section("game", &Colour::new(0, 0, 0));
        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new(&mut self.renderer)));
        //Main loop!
        while self.is_running {
            let input_result: ReturnResult;
            let update_result: ReturnResult;

            //Handle current game state
            match self.state_stack.last_mut() {
                None => return,
                Some(current_state) => {
                    //Drawing happens first because the input is blocking, so nothing would be drawn until input has been
                    //got on the first loop
                    if self.needs_redraw {
                        current_state.draw(&mut self.renderer);
                        self.needs_redraw = false;
                    }
                    //Ensure what has been drawn is flushed to stdout before getting input/updating
                    stdout().flush()
                        .expect("Could not buffer the terminal output!");

                    let input = ui::get_user_input(&self.renderer);
                    let input_args: Vec<&str> = input.trim().split(" ").collect();

                    if input_args.len() == 1 {
                        if input_args[0]  == "exit" {
                            self.is_running = false;
                        }
                    }
                    input_result    = current_state.handle_input(&input_args);
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
            ReturnResult::Redraw => {
                self.renderer.clear_section("debug", &Colour::new(0, 0, 0));
                self.renderer.clear_section("game", &Colour::new(0, 0, 0));
                self.needs_redraw = true;
            }
            ReturnResult::None => {}
        }
    }
}