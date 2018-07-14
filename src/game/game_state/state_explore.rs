use super::GameState;

use ::graphics::renderer::Renderer;
use ::graphics::colour::Colour;

use ::game::player::Player;
use ::game::user_interface as ui;
use ::util::vector::Vector2D;

pub struct StateExplore {
    player: Player,
}

impl StateExplore {
    pub fn new(renderer: &mut Renderer) -> StateExplore {
        let state = StateExplore {
            player: Player::new(),
        };
        ui::reset_ui(renderer);
        state
    }
}

impl GameState for StateExplore {
    fn input(&mut self, renderer: &Renderer) {
        let input = ui::get_user_input(renderer);

        match input.trim() {
            "exit" => {
                renderer.draw_string("debug", "EXIT", &Vector2D::new(2, 2));
            },
            _ => {}
        }
    }

    fn update(&mut self) {

    }

    fn draw(&mut self, renderer: &mut Renderer) {
        
    }
}