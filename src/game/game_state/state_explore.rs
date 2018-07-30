use super::GameState;
use game::UpdateResult;

use graphics::colour::Colour;
use graphics::renderer::Renderer;

use game::player::Player;
use game::world::World;
use game::GAME_AREA_CENTER;

use util::maths::clamp;
use util::vector;
use util::vector::Vector2D;

pub struct StateExplore {
    player: Player,
    world: World,
}

impl StateExplore {
    pub fn new() -> StateExplore {
        StateExplore {
            player: Player::new(),
            world: World::new(),
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
    pub fn handle_move_player_step(&mut self, steps: &str) {
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
        let next_tile = self.world.get_tile(next_position);
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
    fn update(&mut self, input_args: &[&str]) -> Option<UpdateResult> {
        //This is for the player move input, by converting X/Y diretion string to a intergral value
        fn parse_step(n: &str) -> i32 {
            match n.parse::<i32>() {
                Err(_) => 0,
                Ok(step) => step,
            }
        }
        Renderer::set_cursor_location(Vector2D::new(120, 50));
        println!("{}", input_args.len());

        match input_args {
            [steps] => {
                self.handle_move_player_step(steps);
                Some(UpdateResult::Redraw)
            }

            ["x", step] => {
                let step = parse_step(step);
                self.handle_move_player(step, 0);
                Some(UpdateResult::Redraw)
            }

            ["y", step] => {
                let step = parse_step(step);
                self.handle_move_player(0, step);
                Some(UpdateResult::Redraw)
            }

            _ => None,
        }
    }

    /**
     * Draws the player and the overworld etc
     */
    fn draw(&mut self, renderer: &mut Renderer) {
        self.world.render(&renderer, self.player.position);

        renderer.draw_string(
            "debug",
            &self.world.get_tile(self.player.position).to_string(),
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
