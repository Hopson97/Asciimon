mod game_state;
mod player;

pub mod console;
pub mod world;

use graphics::renderer::{Panel, Renderer};
use util::vector::Vector2D;

use self::console::{Console, CONSOLE_WIDTH};
use self::game_state::{state_explore::StateExplore, GameState};

use std::io::{stdin, stdout, Write};

pub const SCREEN_SIZE: Vector2D<i32> = Vector2D { x: 81, y: 52 };

pub const LOGO_POSITION: Vector2D<i32> = Vector2D { x: 0, y: 0 };
pub const LOGO_SIZE: Vector2D<i32> = Vector2D { x: 45, y: 6 };
pub const LOGO: &str = r"
                   _ _
    /\            (_|_)
   /  \   ___  ___ _ _ _ __ ___   ___  _ __
  / /\ \ / __|/ __| | | '_ ` _ \ / _ \| '_ \
 / ____ \ __ \ (__| | | | | | | | (_) | | | |
/_/    \_\___/\___|_|_|_| |_| |_|\___/|_| |_|
";

pub const GAME_AREA_SIZE: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x,
    y: SCREEN_SIZE.y - LOGO_SIZE.y - 1,
};
pub const GAME_AREA_CENTRE: Vector2D<i32> = Vector2D {
    x: GAME_AREA_SIZE.x / 2,
    y: GAME_AREA_SIZE.y / 2,
};
pub const GAME_AREA_POSITION: Vector2D<i32> = Vector2D {
    x: 0,
    y: LOGO_SIZE.y + 1,
};

pub const INPUT_FIELD_SIZE: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x - LOGO_SIZE.x - 2,
    y: LOGO_SIZE.y,
};
pub const INPUT_FIELD_POSITION: Vector2D<i32> = Vector2D {
    x: LOGO_SIZE.x + 2,
    y: 0,
};

pub const CONSOLE_SIZE: Vector2D<i32> = Vector2D {
    x: 32,
    y: SCREEN_SIZE.y,
};
pub const CONSOLE_POSITION: Vector2D<i32> = Vector2D {
    x: SCREEN_SIZE.x + 1,
    y: 0,
};

mod colours {
    use graphics::colour::Colour;
    define_colour!(LOGO, 50, 255, 200);
    define_colour!(TEXT, 255, 255, 255);
    define_colour!(GAME_BACKGROUND, 0, 0, 0);
    define_colour!(UI_BACKGROUND, 50, 50, 50);
}

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
            .add_panel("logo", Panel::new(LOGO_POSITION, LOGO_SIZE));
        game.renderer
            .add_panel("input", Panel::new(INPUT_FIELD_POSITION, INPUT_FIELD_SIZE));
        game.renderer
            .add_panel("game", Panel::new(GAME_AREA_POSITION, GAME_AREA_SIZE));
        game.renderer
            .add_panel("console", Panel::new(CONSOLE_POSITION, CONSOLE_SIZE));

        game.renderer.panel("logo").border();
        game.renderer.panel("input").border();
        game.renderer.panel("game").border();
        game.renderer.panel("console").border();
        game.draw_logo();

        game.renderer.panel("game").clear(&colours::GAME_BACKGROUND);

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
            self.renderer.panel("game").clear(&colours::GAME_BACKGROUND);
            current_state.draw(&mut self.renderer, &mut self.console);

            self.console.draw(self.renderer.panel("console"));
            self.renderer.panel("input").border();

            //Ensure what has been drawn is flushed to stdout before getting input/updating
            stdout()
                .flush()
                .expect("Could not buffer the terminal output!");

            if let Some(input) = Game::get_user_input(&self.renderer) {
                let input_args: Vec<&str> = input.trim().split(' ').collect();

                match &input_args[..] {
                    ["exit"] | ["quit"] => Some(UpdateResult::Exit),
                    ["help"] => {
                        self.console.write(&"-".repeat(CONSOLE_WIDTH as usize - 4));
                        current_state.write_instructions(&mut self.console);
                        self.console.write("Instructions: ");
                        self.console.write(&"-".repeat(CONSOLE_WIDTH as usize - 4));
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

        let input_panel = renderer.panel("input");
        input_panel.clear(&colours::GAME_BACKGROUND);
        input_panel.draw_string("Enter Input Here:", Vector2D::new(0, 1));
        input_panel.draw_string("Enter 'help' for instructions.", Vector2D::new(0, 0));
        input_panel.draw_string("> ", Vector2D::new(0, 2));

        stdout()
            .flush()
            .expect("Could not buffer the terminal output!");

        input_panel.set_cursor(Vector2D::new(2, 2));
        let mut input = String::new();
        match stdin()
            .read_line(&mut input)
            .expect("Failed to get user input")
        {
            0 => None,
            _ => Some(input),
        }
    }

    fn draw_logo(&self) {
        Renderer::set_text_colour(&colours::LOGO);

        let logo_panel = self.renderer.panel("logo");
        logo_panel.clear(&colours::GAME_BACKGROUND);

        for (line_num, line) in LOGO.lines().enumerate() {
            logo_panel.draw_string(line, Vector2D::new(0, line_num as i32 - 1));
        }

        Renderer::set_text_colour(&colours::TEXT);
    }
}
