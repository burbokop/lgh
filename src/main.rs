#![feature(option_result_contains)]

use std::time::Duration;

use fleet::Fleet;
use scenario::scenario1;
use scene::Scene;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode};


mod fleet;
mod scene;
mod admiral;
mod scenario;
mod graphics;
mod visual_effects;
mod rect;
mod side;
mod math;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();
    let mut font = ttf_context.load_font("/usr/share/fonts/truetype/open-sans/OpenSans-Regular.ttf", 12).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut scene = scenario1();



    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;

        let a = ((((i as f64 * 0.01).sin() + 1.) * 0.5 * 64.) as u8) + 64;

        canvas.set_draw_color(Color::RGB(a, 64, 255 - a));
        canvas.clear();



        let events = event_pump.poll_iter().filter(|event| -> bool {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0)
                },
                _ => { true }
            }
        });

        events.for_each(|e| { scene.event(&e); });

        scene.update();

        scene.paint(&mut canvas, &font);

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
