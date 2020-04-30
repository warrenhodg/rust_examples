extern crate rustris;
extern crate sdl2;

use super::Colour;
use super::Game;
use super::Output;

use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;

pub struct Sdl {
    window: Box<sdl2::video::Window>,
    canvas: Box<sdl2::render::WindowCanvas>,
    textris: Box<sdl2::render::Texture>,
}

impl Sdl {
    fn new(ctx: sdl2::Sdl) -> Self {
        let video_subsystem = ctx.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        let screen_size = video_subsystem.display_bounds(0).unwrap();

        let window = video_subsystem.window("rustris", screen_size.width(), screen_size.height())
            .position_centered()
            .fullscreen()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas()
            .build()
            .unwrap();

        let texture_creator = canvas.texture_creator();

        let mut font = ttf_context.load_font("./font.ttf", 72).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let surface = font.render("textris!")
            .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        let textris_texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        Self {
            canvas: canvas,
            textris: textris_texture,
        }
    }
}

impl<'a> Output for Sdl {
    fn reset(&mut self) {
    }

    fn show_message(&mut self, message: String) {
    }

    fn show_game(&mut self, game: &Game) {
    }
}
