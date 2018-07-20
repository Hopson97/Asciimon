use super::GameState;
use super::ReturnResult;

use ::graphics::renderer::Renderer;

use ::game::player::Player;
use ::game::user_interface as ui;
use ::game::map::{Map, MAP_WIDTH, MAP_HEIGHT};
use ::game::{GAME_AREA_X, GAME_AREA_Y};

use ::util::vector::Vector2D;
use ::util::maths::{clamp};

const CENTER_X: u8 = GAME_AREA_X / 2;
const CENTER_Y: u8 = GAME_AREA_Y / 2;

enum Action 
{
    NoAction,
    MovePlayer(i16, i16)
}

pub struct StateExplore {
    player: Player,
    last_action: Action,
    current_map: Map
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let state = StateExplore {
            player:         Player::new(),
            last_action:    Action::NoAction,
            current_map:    Map::load(0, 0).unwrap()
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

    fn draw_map(&self, renderer: &Renderer) {
        let d = self.current_map.data();
        let mut map_x = CENTER_X as i16 - self.player.local_position().x;
        let map_y = CENTER_Y as i16 - self.player.local_position().y;

        let mut begin_slice = 0;
        let mut end_slice = GAME_AREA_X as i16;

        if map_x < 0 {
            begin_slice = map_x.abs();
            map_x = 0;
        }

        if map_x + MAP_WIDTH > GAME_AREA_X as i16 {
            end_slice = GAME_AREA_X as i16 - map_x;
        }

        for y in 0..MAP_HEIGHT {
            renderer.draw_string("game", 
                &d[y as usize][begin_slice as usize..end_slice as usize], 
                &Vector2D::new(map_x, map_y + y));
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
        self.draw_map(&renderer);
        
        //Draw player position
        renderer.draw_string("game", "@", &Vector2D::new(CENTER_X as i16, CENTER_Y as i16));
    }
}