use super::GameState;
use super::ReturnResult;

use ::graphics::renderer::Renderer;
use ::graphics::colour::Colour;

use ::game::player::Player;
use ::game::user_interface as ui;
use ::game::map_manager::MapManager;
use ::game::{GAME_AREA_X, GAME_AREA_Y};

use ::util::vector::Vector2D;
use ::util::maths::{clamp};

pub const CENTER_X: u8 = GAME_AREA_X / 2;
pub const CENTER_Y: u8 = GAME_AREA_Y / 2;

enum Action 
{
    NoAction,
    MovePlayer(i16, i16)
}

pub struct StateExplore {
    player: Player,
    last_action: Action,
    maps: MapManager
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let mut state = StateExplore {
            player:         Player::new(),
            last_action:    Action::NoAction,
            maps:           MapManager::new()
        };

        ui::reset_ui(renderer);
        state
    }

    /**
     * Attempts to the move the player's local position by x/y amount in the x/y direction
     */
    pub fn handle_move_player(&mut self, x_offset: i16, y_offset: i16) {
        let x_move = clamp(x_offset, -1, 1);
        let y_move = clamp(y_offset, -1, 1);

        //@TODO: Handle the "DRY" here
        for _ in 0..x_offset.abs() {
            let p_move = Vector2D::new(x_move, 0);
            let next_position = p_move + self.player.position().clone();
            if self.maps.get_tile(&next_position) == '#' {
                break;
            }
            self.player.move_position(x_move, 0);
        }

        for _ in 0..y_offset.abs() {
            let p_move = Vector2D::new(0, y_move);
            let next_position = p_move + self.player.position().clone();
            if self.maps.get_tile(&next_position) == '#' {
                break;
            }
            self.player.move_position(0, y_move);
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
        self.maps.render_maps(&renderer, &self.player.position());

        renderer.draw_string("debug", &self.maps.get_tile(&self.player.position()).to_string(), &Vector2D::new(0, 0));
        

        //Draw player position
        Renderer::set_text_colour(&Colour::new(0, 153, 175));
        renderer.draw_string("game", "@", &Vector2D::new(CENTER_X as i16, CENTER_Y as i16));
    }
}