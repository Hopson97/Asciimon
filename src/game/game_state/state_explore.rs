use super::GameState;
use super::ReturnResult;

use ::graphics::renderer::Renderer;

use ::game::player::Player;
use ::game::user_interface as ui;
use ::game::map_manager::MapManager;
use ::game::{GAME_AREA_X, GAME_AREA_Y};

use ::util::vector::Vector2D;
use ::util::maths::{clamp};

enum Action 
{
    NoAction,
    MovePlayer(i16, i16)
}

pub struct StateExplore {
    player: Player,
    map_manager: MapManager,
    last_action: Action,
    player_draw_point: Vector2D<u8>,
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let mut state = StateExplore {
            player:             Player::new(),
            map_manager:        MapManager::new(),
            last_action:        Action::NoAction,
            player_draw_point:  Vector2D::new(GAME_AREA_X / 2, GAME_AREA_Y / 2),
        };
        ui::reset_ui(renderer);
        state.map_manager.update_player_pos(state.player.position());
        state
    }

    /**
     * Attempts to the move the player's local position by x/y amount in the x/y direction
     */
    pub fn handle_move_player(&mut self, x: i16, y: i16) {
        let x_move = clamp(x, -1, 1);
        let y_move = -clamp(y, -1, 1);

        for _ in 0..x.abs() {
            self.player.move_position(x_move, 0);
        }

        for _ in 0..y.abs() {
            self.player.move_position(0, y_move);
        }

        self.map_manager.update_player_pos(self.player.position());
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
        let map_lines = self.map_manager.map_lines(self.player.position(), &self.player_draw_point);
        let mut y = 0;
        for line in map_lines {
            renderer.draw_string("debug", &line, &Vector2D::new(0, y));
            y+= 1;
        }

        renderer.draw_string("game", "@", &self.player_draw_point);
        renderer.draw_string("debug", &self.player_draw_point.x.to_string(), &Vector2D::new(0, 5));
        renderer.draw_string("debug", &self.player_draw_point.y.to_string(), &Vector2D::new(0, 6));
    }
}