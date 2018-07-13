mod graphics;

use graphics::renderer::Renderer;

fn main() {
    print!("\x1b[2J");
    let renderer = Renderer::new(64, 32);

    println!("");
    println!("");
}
