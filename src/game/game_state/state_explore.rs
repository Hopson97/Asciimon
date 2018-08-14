use super::GameState;
use game::UpdateResult;

use graphics::Colour;
use graphics::Renderer;

use game::Console;
use game::Player;
use game::World;
use game::GAME_AREA_CENTRE;

use util::{maths::clamp, vector, Vector2D};

mod colours {
    use graphics::Colour;
    define_colour!(PLAYER, 0, 153, 175);
}

pub struct StateExplore {
    player: Player,
    world: World,
    last_move: String,
}

impl StateExplore {
    pub fn new() -> StateExplore {
        StateExplore {
            player: Player::new(),
            world: World::new(),
            last_move: String::new(),
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

    /// Cycles through a buffer of move commands one by one and steps the plyer
    /// #Example
    /// >wwwssdd
    /// Moves player 3 left, then 2 down, then 2 right.
    /// Collision will stop player moving in a direction, but will continue to cycle the buffer
    pub fn handle_move_player_step(&mut self, steps: &str) {
        for step in steps.chars() {
            if !self.move_player(match step {
                'w' => vector::UP,
                'a' => vector::LEFT,
                's' => vector::DOWN,
                'd' => vector::RIGHT,
                _ => continue,
            }) {
                break;
            };
        }
    }

    /// Moves player ~1 step
    /// Returns true if the player moved, false if not (eg collision or other event occurs)
    fn move_player(&mut self, move_amount: Vector2D<i32>) -> bool {
        let next_position = self.player.position() + move_amount;
        match self.world.get_tile(next_position) {
            '.' | ',' | '|' | '\'' | '1' => {
                //Tiles player can walk on
                self.player.move_position(move_amount);
                //Move player to destination portal if there is one
                if self.world.is_portal_at(self.player.position()) {
                    let portal_dest = self.world.get_portal_at(self.player.position());
                    let curr_pos = self.player.position();
                    let offset = portal_dest - curr_pos;
                    self.player.move_position(offset);
                    return false;
                }
                true
            }
            _ => false,
        }
    }
}

impl GameState for StateExplore {
    fn write_instructions(&self, console: &mut Console) {
        define_colour!(TEXT_COL, 100, 255, 110);

        let mut add_instruction = |command: &str, desc: &str, example: &str, example_desc: &str| {
            console.skip_line();
            console.write_with_colour(
                &format!("Example: '{}', {}", example, example_desc),
                TEXT_COL,
            );
            console.write_with_colour(&format!("{} - {}", command, desc), TEXT_COL);
        };

        add_instruction(
            "W/A/S/D",
            "Moves player around world",
            "waass",
            "Move player up, 2 left, 2 down",
        );
        add_instruction(
            "X <n>",
            "Moves player in the x plane up to <n> times",
            "x -10",
            "Moves player 10 tiles to left",
        );
        add_instruction(
            "Y <n>",
            "Moves player in the Y plane up to <n> times",
            "y 10",
            "Moves player 10 tiles up",
        );
    }

    fn execute_command(
        &mut self,
        command_args: &[&str],
        console: &mut Console,
    ) -> Option<UpdateResult> {
        //This is for the player move input, by converting X/Y direction string to a integral value
        fn parse_step(n: &str) -> i32 {
            match n.parse::<i32>() {
                Err(_) => 0,
                Ok(step) => step,
            }
        }

        match command_args {
            [""] => {
                let steps = self.last_move.clone();
                self.handle_move_player_step(&steps);
            }
            [steps] => {
                self.handle_move_player_step(steps);
                self.last_move = steps.to_string();
            }
            ["x", step] => {
                let step = parse_step(step);
                self.handle_move_player(step, 0);
            }
            ["y", step] => {
                let step = parse_step(step);
                self.handle_move_player(0, step);
            }
            _ => {}
        };

        console.write(&format!(
            "Player Postion: {} {}",
            self.player.position().x,
            self.player.position().y
        ));

        None
    }

    ///Draws the player and the overworld etc
    fn draw(&mut self, renderer: &mut Renderer, console: &mut Console) {
        self.world.render(&renderer, self.player.position());
        //Draw player position
        Renderer::set_text_colour(colours::PLAYER);
        renderer.draw_string("game", "@", GAME_AREA_CENTRE);
    }
}
