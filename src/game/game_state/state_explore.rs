use super::GameState;
use super::ReturnResult;

use ::graphics::renderer::Renderer;
use ::graphics::colour::Colour;

use ::game::Game;
use ::game::player::Player;
use ::game::user_interface as ui;

use ::util::vector::Vector2D;
use ::util::maths::{clamp};

enum LastAction 
{
    NoAction,
    MovePlayer(i16, i16)
}

pub struct StateExplore {
    player: Player,
    last_action: LastAction
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let state = StateExplore {
            player: Player::new(),
            last_action: LastAction::NoAction
        };
        ui::reset_ui(renderer);
        state
    }

    pub fn handle_move_player(&mut self, x: i16, y: i16) {
        let x_move = -clamp(x, -1, 1);
        let y_move = -clamp(y, -1, 1);

        for _ in 0..x.abs() {
            self.player.move_local_position(x_move, 0);
        }

        for _ in 0..y.abs() {
            if self.player.local_position().y + y_move < 0 {
                break;
            }
            self.player.move_local_position(0, y_move);
        }
    }
}

impl GameState for StateExplore {
    /**
     * Handles user input for the exploring of the world
     */
    fn input(&mut self, renderer: &Renderer)  -> ReturnResult {
        fn get_step(n: &str) -> i16 {
            match n.parse::<i16>() {
                Err(_) => 0,
                Ok(step) => step
            }
        }

        let input = ui::get_user_input(renderer);
        let input_args: Vec<&str> = input.trim().split(" ").collect();

        if input_args.len() == 2 {
            match input_args[0] {
                "y" => {
                    let step = get_step(input_args[1]);
                    self.last_action = LastAction::MovePlayer(0, step);
                }
                "x" => {
                    let step = get_step(input_args[1]);
                    self.last_action = LastAction::MovePlayer(step, 0);
                }
                _ => {}
            };
        }
        else if input_args.len() == 1 {

        }
        
        match input_args[0] {
            "exit" => {
                return ReturnResult::Exit;
            }
            _ => {}
        };
        ReturnResult::None
    }

    /**
     * Handles the updating of the game for the player
     */
    fn update(&mut self) -> ReturnResult {
        let mut ret_result = ReturnResult::None;
        match self.last_action {
            LastAction::MovePlayer(x, y) => {
                ret_result = ReturnResult::Redraw;
                self.handle_move_player(x, y);
            }
            LastAction::NoAction => {}
        }

        ret_result
    }

    /**
     * Draws the player and the overworld etc
     */
    fn draw(&mut self, renderer: &mut Renderer) {
        let p = &self.player.local_position();
        let draw_pos = Vector2D::new(p.x as u8, p.y as u8);
        renderer.draw_string("game", "@", &draw_pos);
    }
}