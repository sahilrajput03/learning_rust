// this if final code of blinking sqare from pg. 110.
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");
    // Parameters are: title, width, height
    let window = video_subsystem
        .window("Tetris", 800, 600)
        .position_centered() // to put it in the middle of the screen
        .build() // to create the window
        .expect("Failed to create window");
    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync() // To enable v-sync.
        .build()
        .expect("Couldn't get window's canvas");
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    // To make things easier to read, we'll create a constant which will be the texture's size.
    const TEXTURE_SIZE: u32 = 32;
    // We create a texture with a 32x32 size.
    let mut square_texture: Texture = texture_creator
        .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("Failed to create a texture");
    // We use the canvas to draw into our square texture.
    canvas
        .with_texture_canvas(&mut square_texture, |texture| {
            // We set the draw color to green.
            texture.set_draw_color(Color::RGB(0, 255, 0));
            // We "clear" our texture so it'll be fulfilled with green.
            texture.clear();
        })
        .expect("Failed to color a texture");
    // First we get the event handler:
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");
    // Then we create an infinite loop to loop over events:
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // If we receive a 'quit' event or if the user press the 'ESC' key, we quit.
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running; // We "break" the infinite loop.
                }
                _ => {}
            }
        }
        // We set fulfill our window with red.
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        // We draw it.
        canvas.clear();
        // Copy our texture into the window.
        canvas
            .copy(
                &square_texture,
                None,
                // We copy it at the top-left of the window with a 32x32 size.
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("Couldn't copy texture into window");
        // We update window's display.
        canvas.present();
        // We sleep enough to get ~60 fps. If we don't call this, the program will take
        // 100% of a CPU time.
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
