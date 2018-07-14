use ::graphics::renderer::{Renderer};
use ::graphics::colour::Colour;

use ::util::vector::Vector2D;

use std::io;
use std::io::Write;
use std::io::stdout;

pub fn init(renderer: &mut Renderer) {
    let width = renderer.get_size().x;

    renderer.add_render_section("logo", Vector2D::new(0, 0), Vector2D::new(50, 6));
    renderer.add_render_section("input", Vector2D::new(50, 0), Vector2D::new(width - 50, 6));
    reset_ui(renderer)
}

fn draw_logo(renderer: &Renderer) {
    renderer.draw_string("logo", "                    _ _                       ",    &Vector2D::new(0, 0));
    renderer.draw_string("logo", "     /\\            (_|_)                      ",   &Vector2D::new(0, 1));
    renderer.draw_string("logo", "    /  \\   ___  ___ _ _ _ __ ___   ___  _ __  ",   &Vector2D::new(0, 2));
    renderer.draw_string("logo", "   / /\\ \\ / __|/ __| | | '_ ` _ \\ / _ \\| '_ \\ ",    &Vector2D::new(0, 3));
    renderer.draw_string("logo", "  / ____ \\ __ \\ (__| | | | | | | | (_) | | | |",    &Vector2D::new(0, 4));
    renderer.draw_string("logo", " /_/    \\_\\___/\\___|_|_|_| |_| |_|\\___/|_| |_|",    &Vector2D::new(0, 5));
}

pub fn reset_ui(renderer: &mut Renderer) {
    let border_colour = Colour::new(25, 25, 25);

    renderer.clear();
    renderer.draw_solid_line_x(
        &border_colour, 
        &Vector2D::new(0, 6), 
        renderer.get_size().x);

    renderer.draw_solid_line_y(
        &border_colour,
        &Vector2D::new(49, 0),
        6);
    draw_logo(&renderer);
}

pub fn get_user_input(renderer: &Renderer) -> String {
    renderer.clear_section("input");

    renderer.draw_string("input", "Please refer to INSTRUCTIONS.txt for commands.", &Vector2D::new(0, 0));
    renderer.draw_string("input", "Enter Input Here:", &Vector2D::new(0, 1));
    renderer.draw_string("input", "> ",                &Vector2D::new(0, 2));

    stdout().flush()
        .expect("Could not buffer the terminal output!");

    renderer.set_cursor_render_section("input", &Vector2D::new(2, 2));
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get user input");
    input
}
