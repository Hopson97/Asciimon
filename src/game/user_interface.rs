use ::graphics::renderer::{Renderer, RenderSection};
use ::graphics::colour::Colour;

use ::util::vector::Vector2D;

pub const GAME_BEGIN_Y: u8 = 6;
const INPUT_AREA_X: u8 = 50;

pub fn reset_ui(renderer: &mut Renderer) {
    let border_colour = Colour::new(25, 25, 25);

    renderer.clear(&Colour::new(25, 25, 50));
    renderer.draw_solid_line_x(
        &border_colour, 
        &Vector2D::new(0, GAME_BEGIN_Y), 
        renderer.get_size().x);

    renderer.draw_solid_line_y(
        &border_colour,
        &Vector2D::new(INPUT_AREA_X - 1, 0),
        GAME_BEGIN_Y);
    draw_logo(&renderer);

    renderer.draw_string(RenderSection::InputArea, "Please refer to INSTRUCTIONS.txt for commands", &Vector2D::new(INPUT_AREA_X, 0));
    renderer.draw_string(RenderSection::InputArea, "Enter Input Here:", &Vector2D::new(INPUT_AREA_X, 1));
    renderer.draw_string(RenderSection::InputArea, "> ",                &Vector2D::new(INPUT_AREA_X, 2));
}

fn draw_logo(renderer: &Renderer) {
    renderer.draw_string(RenderSection::InputArea, "                    _ _                       ",    &Vector2D::new(0, 0));
    renderer.draw_string(RenderSection::InputArea, "     /\\            (_|_)                      ",   &Vector2D::new(0, 1));
    renderer.draw_string(RenderSection::InputArea, "    /  \\   ___  ___ _ _ _ __ ___   ___  _ __  ",   &Vector2D::new(0, 2));
    renderer.draw_string(RenderSection::InputArea, "   / /\\ \\ / __|/ __| | | '_ ` _ \\ / _ \\| '_ \\ ",    &Vector2D::new(0, 3));
    renderer.draw_string(RenderSection::InputArea, "  / ____ \\ __ \\ (__| | | | | | | | (_) | | | |",    &Vector2D::new(0, 4));
    renderer.draw_string(RenderSection::InputArea, " /_/    \\_\\___/\\___|_|_|_| |_| |_|\\___/|_| |_|",    &Vector2D::new(0, 5));
}

fn get_user_input(renderer: &mut Renderer) -> String {
    let input = String::new();


    input
}
