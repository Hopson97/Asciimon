use super::GameState;

use ::graphics::renderer::Renderer;

use ::game::player::Player;

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

    }

    fn update(&mut self) {

    }

    fn draw(&mut self, renderer: &mut Renderer) {
        
    }
}