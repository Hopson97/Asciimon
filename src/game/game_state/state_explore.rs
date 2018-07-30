use super::GameState;
use super::ReturnResult;

use graphics::colour::Colour;
use graphics::renderer::Renderer;

use game::map_manager::MapManager;
use game::player::Player;
use game::GAME_AREA_CENTER;

use util::maths::clamp;
use util::vector;
use util::vector::Vector2D;

#[derive(Clone)]
enum Action {
    None,
    MovePlayer(i32, i32),
    MovePlayerStep(String),
}

pub struct StateExplore {
    player: Player,
    next_action: Action,
    maps: MapManager,
}

impl StateExplore {
    pub fn new() -> StateExplore {
        StateExplore {
            player: Player::new(),
            next_action: Action::None,
            maps: MapManager::new(),
        }
    }

    ///Attempts to the move the player's local position by x/y amount in the x/y direction
    pub fn handle_move_player(&mut self, x_offset: i32, y_offset: i32) {
        let x_move = clamp(x_offset, -1, 1);
        let y_move = -clamp(y_offset, -1, 1);
        let move_vector = Vector2D::new(x_move, y_move);

        for _ in 0..x_offset.abs() {
            if !self.move_player(move_vector) {
                break;
            }
        }

        for _ in 0..y_offset.abs() {
            if !self.move_player(move_vector) {
                break;
            }
        }
    }

    ///Cycles through a buffer of move commands one by one and steps the plyer
    /// #Example
    /// >wwwssdd
    /// Moves player 3 left, then 2 down, then 2 right.
    /// Collision will stop player moving in a direction, but will continue to cycle the buffer
    pub fn handle_move_player_step(&mut self, steps: &String) {
        for step in steps.chars() {
            self.move_player(match step {
                'w' => vector::UP,
                'a' => vector::LEFT,
                's' => vector::DOWN,
                'd' => vector::RIGHT,
                _ => continue,
            });
        }
    }

    fn move_player(&mut self, move_amount: Vector2D<i32>) -> bool {
        let next_position = self.player.position + move_amount;
        let next_tile = self.maps.get_tile(next_position);
        if next_tile == '0' || next_tile == 'Y' || next_tile == '~' {
            false
        } else {
            self.player.move_position(move_amount);
            true
        }
    }
}

impl GameState for StateExplore {
    /**
     * Handles user input for the exploring of the world
     */
    fn handle_input(&mut self, input_args: &[&str]) -> ReturnResult {
        //This is for the player move input, by converting X/Y diretion string to a intergral value
        fn get_step(n: &str) -> i32 {
            match n.parse::<i32>() {
                Err(_) => 0,
                Ok(step) => step,
            }
        }
        Renderer::set_cursor_location(Vector2D::new(120, 50));
        println!("{}", input_args.len());

        // self.next_action = Action::None; //Reset last action so it does not get repeated

        if input_args.len() == 1 {
            match input_args[0].chars().next() {
                None => {}
                Some(c) => match c {
                    'w' => self.next_action = Action::MovePlayerStep(String::from(input_args[0])),
                    'a' => self.next_action = Action::MovePlayerStep(String::from(input_args[0])),
                    's' => self.next_action = Action::MovePlayerStep(String::from(input_args[0])),
                    'd' => self.next_action = Action::MovePlayerStep(String::from(input_args[0])),
                    _ => {}
                },
            };
        } else if input_args.len() == 2 {
            match input_args[0] {
                "y" => {
                    let step = get_step(input_args[1]);
                    self.next_action = Action::MovePlayer(0, step);
                }
                "x" => {
                    let step = get_step(input_args[1]);
                    self.next_action = Action::MovePlayer(step, 0);
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
        match self.next_action.clone() {
            Action::None => ReturnResult::None,
            Action::MovePlayer(x, y) => {
                self.handle_move_player(x, y);
                ReturnResult::Redraw
            }
            Action::MovePlayerStep(steps) => {
                self.handle_move_player_step(&steps);
                ReturnResult::Redraw
            }
        }
    }

    /**
     * Draws the player and the overworld etc
     */
    fn draw(&mut self, renderer: &mut Renderer) {
        self.maps.render_maps(&renderer, self.player.position);

        renderer.draw_string(
            "debug",
            &self.maps.get_tile(self.player.position).to_string(),
            Vector2D::new(0, 0),
        );
        renderer.draw_string(
            "debug",
            &self.player.position.x.to_string(),
            Vector2D::new(0, 1),
        );
        renderer.draw_string(
            "debug",
            &self.player.position.y.to_string(),
            Vector2D::new(5, 1),
        );

        //Draw player position
        Renderer::set_text_colour(&Colour::new(0, 153, 175));
        renderer.draw_string("game", "@", GAME_AREA_CENTER);
    }
}
