use super::GameState;

use ::graphics::renderer::Renderer;
use ::graphics::colour::Colour;

use ::game::player::Player;
use ::util::vector::Vector2D;

use std::io;

pub struct StateExplore {
    player: Player
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let state = StateExplore {
            player: Player::new()
        };
        renderer.draw_solid_line_x(&Colour::new(20, 20, 20), &Vector2D::new(0, 10), 96);

        state
    }
}

impl GameState for StateExplore {
    fn input(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("oh no");
    }

    fn update(&mut self) {

    }

    fn draw(&mut self, renderer: &mut Renderer) {
        
    }
}