mod game;
mod graphics;
mod util;

use game::Game;

fn clear_terminal() {
    print!("\x1b[2J");
}

fn main() {
    clear_terminal();

    Game::run_game();
    clear_terminal();

    //Ensure terminal is below the game after exiting
    println!();
    println!();
}
