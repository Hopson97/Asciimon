use super::GameState;
use super::ReturnResult;

use ::graphics::renderer::Renderer;

use ::game::player::Player;
use ::game::user_interface as ui;

use ::util::vector::Vector2D;
use ::util::maths::{clamp};

enum Action 
{
    NoAction,
    MovePlayer(i16, i16)
}

pub struct StateExplore {
    player: Player,
    last_action: Action
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let state = StateExplore {
            player: Player::new(),
            last_action: Action::NoAction
        };
        ui::reset_ui(renderer);
        state
    }

    /**
     * Attempts to the move the player's local position by x/y amount in the x/y direction
     */
    pub fn handle_move_player(&mut self, x: i16, y: i16) {
        let x_move = clamp(x, -1, 1);
        let y_move = -clamp(y, -1, 1);

        for _ in 0..x.abs() {
            self.player.move_local_position(x_move, 0);
        }

        for _ in 0..y.abs() {
            self.player.move_local_position(0, y_move);
        }
    }
}

impl GameState for StateExplore {
    /**
     * Handles user input for the exploring of the world
     */
    fn handle_input(&mut self, input_args: &Vec<&str>)  -> ReturnResult {
        //This is for the player move input, by converting X/Y diretion string to a intergral value
        fn get_step(n: &str) -> i16 {
            match n.parse::<i16>() {
                Err(_) => 0,
                Ok(step) => step
            }
        }

        self.last_action = Action::NoAction; //Reset last action so it does not get repeated

        if input_args.len() == 2 {
            match input_args[0] {
                "y" => {
                    let step = get_step(input_args[1]);
                    self.last_action = Action::MovePlayer(0, step);
                }
                "x" => {
                    let step = get_step(input_args[1]);
                    self.last_action = Action::MovePlayer(step, 0);
                }
                _ => {}
            };
        }
        ReturnResult::None
    }

    /**
     * Handles the updating of the game for the player
     */
    fn update(&mut self) -> ReturnResult {
        let mut ret_result = ReturnResult::None;
        match self.last_action {
            Action::MovePlayer(x, y) => {
                ret_result = ReturnResult::Redraw;
                self.handle_move_player(x, y);
            }
            Action::NoAction => {}
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