use super::GameState;
use super::ReturnResult;

use ::graphics::renderer::Renderer;
use ::graphics::colour::Colour;

use ::game::Game;
use ::game::player::Player;
use ::game::user_interface as ui;
use ::util::vector::Vector2D;

pub struct StateExplore {
    player: Player,
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let state = StateExplore {
            player: Player::new(),
        };
        ui::reset_ui(renderer);
        state
    }
}

impl GameState for StateExplore {
    fn input(&mut self, renderer: &Renderer)  -> ReturnResult {
        let input = ui::get_user_input(renderer);
        let input_args: Vec<&str> = input.trim().split(" ").collect();
        
        match input_args[0] {
            "n" => {

            }
            "exit" => {
                return ReturnResult::Exit;
            }
            _ => {}
        };
        ReturnResult::None
    }

    fn update(&mut self) -> ReturnResult {
        ReturnResult::None
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_string("game", "@", &self.player.get_local_position());
    }
}