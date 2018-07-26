mod game_state;
mod player;

pub mod map;
pub mod map_manager;

use ::graphics::colour::Colour;
use ::graphics::renderer::Renderer;
use ::util::vector::Vector2D;

use self::game_state::ReturnResult;
use self::game_state::GameState;
use self::game_state::state_explore::StateExplore;

use std::io::Write;
use std::io::stdin;
use std::io::stdout;

pub const GAME_AREA_X: i32 = 81;
pub const GAME_AREA_Y: i32 = 45;

pub struct Game {
    renderer: Renderer,
    state_stack: Vec<Box<GameState>>,
    is_running: bool,
    needs_redraw: bool
}

impl Game {
    pub fn run_game() {
        let mut game = Game {
            renderer: Renderer::new(81, 52),
            state_stack: Vec::new(),
            is_running: true,
            needs_redraw: true
        };
        //ui::init(&mut game.renderer);

        //Yay for magic numbers
        game.renderer.add_render_section("game",    Vector2D::new(0, 7),    Vector2D::new(GAME_AREA_X, GAME_AREA_Y));
        game.renderer.add_render_section("logo",    Vector2D::new(0, 0),    Vector2D::new(50, 6));
        game.renderer.add_render_section("input",   Vector2D::new(50, 0),   Vector2D::new(GAME_AREA_X - 50, 6));

        game.renderer.create_border("game");
        game.renderer.create_border("logo");
        game.renderer.create_border("input");

        Game::draw_logo(&game.renderer);

        game.run();
    }

    fn run(&mut self) {
        self.state_stack.push(Box::new(StateExplore::new()));
        //Main loop!
        while self.is_running {
            let input_result: ReturnResult;
            let update_result: ReturnResult;

            //Handle current game state
            match self.state_stack.last_mut() {
                None => return,
                Some(current_state) => {
                    //Drawing happens first because the input is blocking, so nothing would be drawn until input has been
                    //got on the first loop
                    if self.needs_redraw {
                        current_state.draw(&mut self.renderer);
                        self.needs_redraw = false;
                    }
                    //Ensure what has been drawn is flushed to stdout before getting input/updating
                    stdout().flush()
                        .expect("Could not buffer the terminal output!");

                    let input = Game::get_user_input(&self.renderer);
                    let input_args: Vec<&str> = input.trim().split(" ").collect();

                    if input_args.len() == 1 {
                        if input_args[0]  == "exit" {
                            self.is_running = false;
                        }
                    }
                    input_result    = current_state.handle_input(&input_args);
                    update_result   = current_state.update();
                }
            } 
            self.handle_return_result(input_result);
            self.handle_return_result(update_result);
        }
    }

    fn handle_return_result(&mut self, input_result: ReturnResult) {
        match input_result {
            ReturnResult::Exit => self.is_running = false,
            ReturnResult::StatePop => {
                self.state_stack.pop();
                if self.state_stack.is_empty() {
                    self.is_running = false;
                }
            },
            ReturnResult::StatePush(state) => {
                self.state_stack.push(state);
            },
            ReturnResult::Redraw => {
                self.renderer.clear_section("debug", &Colour::new(0, 0, 0));
                self.renderer.clear_section("game", &Colour::new(0, 0, 0));
                self.needs_redraw = true;
            }
            ReturnResult::None => {}
        }
    }

    pub fn get_user_input(renderer: &Renderer) -> String {
        Renderer::set_text_colour(&Colour::new(255, 255, 255));
        renderer.clear_section("input", renderer.default_clear_colour());
        renderer.draw_string("input", "Enter Input Here:", &Vector2D::new(0, 0));
        renderer.draw_string("input", "> ",                &Vector2D::new(0, 2));

        stdout().flush()
            .expect("Could not buffer the terminal output!");

        renderer.set_cursor_render_section("input", &Vector2D::new(2, 2));
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to get user input");
        input
    }

    fn draw_logo(renderer: &Renderer) {
        Renderer::set_text_colour(&Colour::new(50, 255, 200));
        renderer.draw_string("logo", "                    _ _                       ",      &Vector2D::new(0, 0));
        renderer.draw_string("logo", "     /\\            (_|_)                      ",     &Vector2D::new(0, 1));
        renderer.draw_string("logo", "    /  \\   ___  ___ _ _ _ __ ___   ___  _ __  ",     &Vector2D::new(0, 2));
        renderer.draw_string("logo", "   / /\\ \\ / __|/ __| | | '_ ` _ \\ / _ \\| '_ \\ ", &Vector2D::new(0, 3));
        renderer.draw_string("logo", "  / ____ \\ __ \\ (__| | | | | | | | (_) | | | |",    &Vector2D::new(0, 4));
        renderer.draw_string("logo", " /_/    \\_\\___/\\___|_|_|_| |_| |_|\\___/|_| |_|",  &Vector2D::new(0, 5));
        Renderer::set_text_colour(&Colour::new(255, 255, 255));
    }
}