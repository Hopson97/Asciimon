use super::GameState;

use ::graphics::renderer::Renderer;

use ::game::player::Player;

use std::io;

pub struct StateExplore {
    player: Player
}

impl StateExplore {
    pub fn new() -> StateExplore {
        StateExplore {
            player: Player::new()
        }
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