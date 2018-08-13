extern crate termion;

#[macro_use]
mod graphics;
mod game;
mod util;

use game::Game;

fn main() {
    use termion::clear::All as Clear;
    use termion::color::{Bg, Fg, Reset};
    use termion::cursor::Goto;

    print!("{}", Clear);
    Game::run_game();
    print!("{}{}{}{}", Fg(Reset), Bg(Reset), Goto(1, 1), Clear);
}
