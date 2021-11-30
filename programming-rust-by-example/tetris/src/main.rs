extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{thread::sleep, time::Duration};
pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed"); // initializing sdl2 cotext.
    let video_subsystem = sdl_context
        .video() // getting video subsystem from context ~sahil
        .expect("Couldn't get SDL video subsystem");
    let window = video_subsystem
        .window("rust-sdl2 demo: Video1", 800, 600)
        .position_centered() // this gets the window in the center of screen.
        .opengl() // this makes sdl2 make use of opengl to render.
        .build() // creates the windows using all previous given methods.
        .expect("Failed to create window"); // panics with the given message if error occurs.
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert window into canvas");
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context
        .event_pump() // we get the event handler by this message.
        .expect("Failed to get SDL event pump");
    'running: loop {
        // loop is a keyword which llows creating infinte loops.
        // intereting feature: we added the label ``running`` to the main loop.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
