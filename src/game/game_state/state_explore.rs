use super::GameState;
use super::ReturnResult;

use ::graphics::renderer::Renderer;
use ::graphics::colour::Colour;

use ::game::player::Player;
use ::game::map_manager::MapManager;
use ::game::{GAME_AREA_X, GAME_AREA_Y};

use ::util::vector::Vector2D;
use ::util::maths::{clamp};

pub const CENTER_X: i32 = GAME_AREA_X / 2;
pub const CENTER_Y: i32 = GAME_AREA_Y / 2;

enum Action 
{
    NoAction,
    MovePlayer(i32, i32)
}

pub struct StateExplore {
    player: Player,
    last_action: Action,
    maps: MapManager
}

impl StateExplore {
    pub fn new() -> StateExplore {
        StateExplore {
            player:         Player::new(),
            last_action:    Action::NoAction,
            maps:           MapManager::new()
        }
    }

    /**
     * Attempts to the move the player's local position by x/y amount in the x/y direction
     */
    pub fn handle_move_player(&mut self, x_offset: i32, y_offset: i32) {
        let x_move =  clamp(x_offset, -1, 1);
        let y_move = -clamp(y_offset, -1, 1);

        //@TODO: Handle the "DRY" here
        for _ in 0..x_offset.abs() {
            let p_move = Vector2D::new(x_move, 0);
            let next_position = p_move + self.player.position().clone();
            let next_tile = self.maps.get_tile(&next_position);
            if next_tile == '0' || next_tile == 'Y' || next_tile == '~' {
                break;
            }
            self.player.move_position(x_move, 0);
        }

        for _ in 0..y_offset.abs() {
            let p_move = Vector2D::new(0, y_move);
            let next_position = p_move + self.player.position().clone();
            let next_tile = self.maps.get_tile(&next_position);
            if next_tile == '0' || next_tile == 'Y' || next_tile == '~' {
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
    fn handle_input(&mut self, input_args: &[&str])  -> ReturnResult {
        //This is for the player move input, by converting X/Y diretion string to a intergral value
        fn get_step(n: &str) -> i32 {
            match n.parse::<i32>() {
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
        match self.last_action {
            Action::NoAction => ReturnResult::None,
            Action::MovePlayer(x, y) => {
                self.handle_move_player(x, y);
                ReturnResult::Redraw
            }
    }
    }

    /**
     * Draws the player and the overworld etc
     */
    fn draw(&mut self, renderer: &mut Renderer) {
        self.maps.render_maps(&renderer, &self.player.position());

        renderer.draw_string("debug", &self.maps.get_tile(&self.player.position()).to_string(), &Vector2D::new(0, 0));
        renderer.draw_string("debug", &self.player.position().x.to_string(), &Vector2D::new(0, 1));
        renderer.draw_string("debug", &self.player.position().y.to_string(), &Vector2D::new(5, 1));
        

        //Draw player position
        Renderer::set_text_colour(&Colour::new(0, 153, 175));
        renderer.draw_string("game", "@", &Vector2D::new(CENTER_X, CENTER_Y));
    }
}