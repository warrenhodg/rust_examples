extern crate sdl2;

/*
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::render::CanvasBuilder;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;
use sdl2::surface::SurfaceContext;
use sdl2::video::WindowContext;
use std::time::Duration;

use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
*/

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> sdl2::rect::Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (cons_width as i32 - w) / 2;
    let cy = (cons_height as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn main() {
    let sdl_context :sdl2::Sdl = sdl2::init().unwrap();

    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();

    let screen_size: sdl2::rect::Rect = video_subsystem.display_bounds(0).unwrap();

    let window: sdl2::video::Window = video_subsystem.window("rustris", screen_size.width(), screen_size.height())
        .position_centered()
        .fullscreen()
        .opengl()
        .build()
        .unwrap();

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = sdl2::render::CanvasBuilder::new(window)
        .build()
        .unwrap();

    let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();

    let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init().unwrap();

    let mut font: sdl2::ttf::Font = ttf_context.load_font("./font.ttf", 72).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let surface: sdl2::surface::Surface = font.render("textris!")
        .blended(sdl2::pixels::Color::RGBA(255, 0, 0, 255)).unwrap();
    let textris_texture: sdl2::render::Texture  = texture_creator.create_texture_from_surface(surface).unwrap();
        
    let mut event_pump = sdl_context.event_pump()
        .unwrap();

    let sdl2::render::TextureQuery{width, height, .. } = textris_texture.query();
    let padding = 64;
    let target: sdl2::rect::Rect = get_centered_rect(width, height, screen_size.width() - padding, screen_size.height() - padding);

    let mut i: i16 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | 
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => {
                    break 'running
                }

                _ => {}
            }
        }

        i = (i + 1) & 255;

        canvas.set_draw_color(sdl2::pixels::Color::RGB(i as u8, 64, 255 - i as u8));
        canvas.clear();

        canvas.copy(&textris_texture, None, target).unwrap();

        canvas.present();
        ::std::thread::sleep(::std::time::Duration::from_nanos(1_000_000_000 / 60));
    }
}
