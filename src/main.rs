mod graphics;
mod game;

use game::Game;

fn main() {
    print!("\x1b[2J");
    
    Game::run_game();

    println!("");
    println!("");
}
