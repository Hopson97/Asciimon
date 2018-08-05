mod game_state;
mod player;

pub mod console;
pub mod world;

use graphics::renderer::Renderer;
use util::vector::Vector2D;

use self::console::Console;
use self::game_state::{state_explore::StateExplore, GameState};

use std::io::{stdin, stdout, Write};

pub const SCREEN_SIZE: Vector2D<i32> = Vector2D { x: 120, y: 52 };

pub const LOGO_POSITION: Vector2D<i32> = Vector2D { x: 0, y: 0 };
pub const LOGO_SIZE: Vector2D<i32> = Vector2D { x: 45, y: 6 };

pub const GAME_AREA_POSITION: Vector2D<i32> = Vector2D {
    x: 0,
    y: LOGO_SIZE.y + 1,
};
pub const GAME_AREA_SIZE: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x - (CONSOLE_SIZE.x + 1),
    y: SCREEN_SIZE.y - (LOGO_SIZE.y + 1),
};
pub const GAME_AREA_CENTRE: Vector2D<i32> = Vector2D {
    x: GAME_AREA_SIZE.x / 2,
    y: GAME_AREA_SIZE.y / 2,
};

pub const INPUT_FIELD_POSITION: Vector2D<i32> = Vector2D {
    x: LOGO_SIZE.x + 1,
    y: 0,
};
pub const INPUT_FIELD_SIZE: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x - (LOGO_SIZE.x + 1) - (CONSOLE_SIZE.x + 1),
    y: LOGO_SIZE.y,
};

pub const CONSOLE_POSITION: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x - CONSOLE_SIZE.x,
    y: 0,
};
pub const CONSOLE_SIZE: Vector2D<i32> = Vector2D {
    x: 32,
    y: SCREEN_SIZE.y,
};

mod colours {
    use graphics::colour::Colour;
    define_colour!(LOGO, 50, 255, 200);
    define_colour!(TEXT, 255, 255, 255);
    define_colour!(GAME_BACKGROUND, 0, 0, 0);
    define_colour!(UI_BACKGROUND, 50, 50, 50);
}

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
    StatePush(Box<GameState>),
    TransitionPush(Box<GameState>),
    StatePop,
    Exit,
}

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>,
    is_running: bool,
    console: Console,
}

impl Game {
    pub fn run_game() {
        let mut game = Game {
            renderer: Renderer::new(SCREEN_SIZE),
            state_stack: Vec::new(),
            is_running: true,
            console: Console::new(),
        };

        game.renderer
            .add_render_section("game", GAME_AREA_POSITION, GAME_AREA_SIZE);
        game.renderer
            .add_render_section("logo", LOGO_POSITION, LOGO_SIZE);
        game.renderer
            .add_render_section("input", INPUT_FIELD_POSITION, INPUT_FIELD_SIZE);
        game.renderer
            .add_render_section("console", CONSOLE_POSITION, CONSOLE_SIZE);

        Game::draw_logo(&game.renderer);

        game.renderer
            .clear_section("game", &colours::GAME_BACKGROUND);

        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new()));
        //Main loop!
        while self.is_running {
            match self.tick() {
                Some(UpdateResult::StatePush(state)) => {
                    self.state_stack.push(state);
                }
                Some(UpdateResult::TransitionPush(state)) => {}
                Some(UpdateResult::StatePop) => {
                    self.state_stack.pop();
                    if self.state_stack.is_empty() {
                        self.is_running = false;
                    }
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
            self.renderer
                .clear_section("game", &colours::GAME_BACKGROUND);
            current_state.draw(&mut self.renderer, &mut self.console);
            //Ensure what has been drawn is flushed to stdout before getting input/updating
            stdout()
                .flush()
                .expect("Could not buffer the terminal output!");

            self.console.draw(&mut self.renderer);
            self.renderer.create_border("input");

            if let Some(input) = Game::get_user_input(&self.renderer) {
                let input_args: Vec<&str> = input.trim().split(' ').collect();

                match &input_args[..] {
                    ["exit"] | ["quit"] => Some(UpdateResult::Exit),
                    ["help"] => {
                        self.console.write(&"-".repeat(CONSOLE_SIZE.x as usize - 4));
                        current_state.write_instructions(&mut self.console);
                        self.console.write("Instructions: ");
                        self.console.write(&"-".repeat(CONSOLE_SIZE.x as usize - 4));
                        None
                    }
                    input => current_state.execute_command(input, &mut self.console),
                }
            } else {
                return Some(UpdateResult::Exit);
            }
        } else {
            return Some(UpdateResult::Exit);
        }
    }

    fn get_user_input(renderer: &Renderer) -> Option<String> {
        Renderer::set_text_colour(&colours::TEXT);
        renderer.clear_section("input", &colours::GAME_BACKGROUND);
        renderer.draw_string("input", "Enter Input Here:", Vector2D::new(0, 1));
        renderer.draw_string(
            "input",
            "Enter 'help' for instructions.",
            Vector2D::new(0, 0),
        );
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
        renderer.clear_section("logo", &colours::GAME_BACKGROUND);
        Renderer::set_text_colour(&colours::LOGO);
        for (line_num, line) in LOGO.lines().enumerate() {
            renderer.draw_string("logo", line, Vector2D::new(0, line_num as i32 - 1));
        }
        Renderer::set_text_colour(&colours::TEXT);
    }
}
