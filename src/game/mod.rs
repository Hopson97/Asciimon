mod game_state;
mod player;

pub mod map;
pub mod map_manager;

use graphics::colour::Colour;
use graphics::renderer::Renderer;
use util::vector::Vector2D;

use self::game_state::{state_explore::StateExplore, GameState};

use std::io::{stdin, stdout, Write};

pub const GAME_AREA_SIZE: Vector2D<i32> = Vector2D { x: 81, y: 45 };

pub const GAME_AREA_CENTER: Vector2D<i32> = Vector2D {
    x: GAME_AREA_SIZE.x / 2,
    y: GAME_AREA_SIZE.y / 2,
};

pub const LOGO: &str = r"
                   _ _
    /\            (_|_)
   /  \   ___  ___ _ _ _ __ ___   ___  _ __
  / /\ \ / __|/ __| | | '_ ` _ \ / _ \| '_ \
 / ____ \ __ \ (__| | | | | | | | (_) | | | |
/_/    \_\___/\___|_|_|_| |_| |_|\___/|_| |_|
";

#[allow(dead_code)]
pub enum UpdateResult {
    Redraw,
    StatePush(Box<GameState>),
    StatePop,
    Exit,
}

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>,
    is_running: bool,
    needs_redraw: bool,
}

impl Game {
    pub fn run_game() {
        let mut game = Game {
            renderer: Renderer::new(Vector2D::new(81, 52)),
            state_stack: Vec::new(),
            is_running: true,
            needs_redraw: true,
        };
        //ui::init(&mut game.renderer);

        //Yay for magic numbers
        game.renderer
            .add_render_section("game", Vector2D::new(0, 7), GAME_AREA_SIZE);
        game.renderer
            .add_render_section("logo", Vector2D::new(0, 0), Vector2D::new(50, 6));
        game.renderer.add_render_section(
            "input",
            Vector2D::new(50, 0),
            Vector2D::new(GAME_AREA_SIZE.x - 50, 6),
        );

        game.renderer.create_border("game");
        game.renderer.create_border("logo");
        game.renderer.create_border("input");
        Game::draw_logo(&game.renderer);

        game.renderer
            .clear_section("game", &Colour::new(10, 10, 10));

        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new()));
        //Main loop!
        while self.is_running {
            match self.tick() {
                Some(UpdateResult::Redraw) => {
                    self.renderer.clear_section("debug", &Colour::new(0, 0, 0));
                    self.renderer.clear_section("game", &Colour::new(0, 0, 0));
                    self.needs_redraw = true;
                }
                Some(UpdateResult::StatePush(state)) => {
                    self.state_stack.push(state);
                    self.needs_redraw = true;
                }
                Some(UpdateResult::StatePop) => {
                    self.state_stack.pop();
                    if self.state_stack.is_empty() {
                        self.is_running = false;
                    }
                    self.needs_redraw = true;
                }
                Some(UpdateResult::Exit) => self.is_running = false,
                None => {}
            }
        }
    }

    fn tick(&mut self) -> Option<UpdateResult> {
        if let Some(current_state) = self.state_stack.last_mut() {
            //Drawing happens first because the input is blocking, so nothing would be drawn until input has been
            //got on the first loop
            if self.needs_redraw {
                current_state.draw(&mut self.renderer);
                self.needs_redraw = false;

                //Ensure what has been drawn is flushed to stdout before getting input/updating
                stdout()
                    .flush()
                    .expect("Could not buffer the terminal output!");
            }

            if let Some(input) = Game::get_user_input(&self.renderer) {
                let input_args: Vec<&str> = input.trim().split(' ').collect();
                match &input_args[..] {
                    ["exit"] | ["quit"] => Some(UpdateResult::Exit),
                    input => current_state.update(input),
                }
            } else {
                return Some(UpdateResult::Exit);
            }
        } else {
            return Some(UpdateResult::Exit);
        }
    }

    fn get_user_input(renderer: &Renderer) -> Option<String> {
        Renderer::set_text_colour(&Colour::new(255, 255, 255));
        renderer.clear_section("input", renderer.default_clear_colour());
        renderer.draw_string("input", "Enter Input Here:", Vector2D::new(0, 0));
        renderer.draw_string("input", "> ", Vector2D::new(0, 2));

        stdout()
            .flush()
            .expect("Could not buffer the terminal output!");

        renderer.set_cursor_render_section("input", Vector2D::new(2, 2));
        let mut input = String::new();
        match stdin()
            .read_line(&mut input)
            .expect("Failed to get user input")
        {
            0 => None,
            _ => Some(input),
        }
    }

    fn draw_logo(renderer: &Renderer) {
        Renderer::set_text_colour(&Colour::new(50, 255, 200));
        for (line_num, line) in LOGO.lines().enumerate() {
            renderer.draw_string("logo", line, Vector2D::new(1, line_num as i32));
        }
        Renderer::set_text_colour(&Colour::new(255, 255, 255));
    }
}
