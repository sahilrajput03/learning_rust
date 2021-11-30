const TEXTURE_SIZE: u32 = 32;

extern crate sdl2;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    // We'll want to handle failures outside of this function.
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed"); // initializing sdl2 cotext.
    let video_subsystem = sdl_context
        .video() // getting video subsystem from context ~sahil
        .expect("Couldn't get SDL video subsystem");

    // Parameters are: title, width, height

    let window = video_subsystem
        .window("Tetris", 800, 600)
        .position_centered() // to put it in the middle of the screen
        .build() // to create the window
        .expect("Failed to create window");

    // old-plain red canvas code.
    // let mut canvas = window
    //     .into_canvas()
    //     .build()
    //     .expect("Failed to convert window into canvas");
    // canvas.set_draw_color(Color::RGB(255, 0, 0));
    // canvas.clear();
    // canvas.present();

    let mut canvas = window
        .into_canvas() // transforms the window into a canvas so that we can manipulate it more easily
        .target_texture() // activates texture rendering support
        .present_vsync() // enables the v-sync (also known as verticalsynchronization) limit
        .build() // creates the canvas by applying all previously set parameters
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // We create a texture with a 32x32 size.
    // let mut square_texture: Texture = texture_creator
    //     .create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
    //     .expect("Failed to create a texture");

    let green_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");

    let mut blue_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");

    let timer = SystemTime::now();

    // We use the canvas to draw into our square texture-for-older no-blink code.
    // canvas
    //     .with_texture_canvas(&mut square_texture, |texture| {
    //         // We set the draw color to green.
    //         texture.set_draw_color(Color::RGB(0, 255, 0));
    //         texture.clear() // washes/clears the texture so it'll be filled with green.
    //     })
    //     .expect("Failed to color a texture.");

    // First we get the event handler:
    let mut event_pump = sdl_context
        .event_pump() // we get the event handler by this message.
        .expect("Failed to get SDL event pump");

    // Then we create an infinite loop to loop over events:
    'running: loop {
        // loop is a keyword which allows creating infinte loops, you can add to for and while loops as well.
        // intereting feature: we added the label ``running`` to the main loop.
        for event in event_pump.poll_iter() {
            match event {
                // If we receive a 'quit' event or if the user press the 'ESC' key, we quit.
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // sleep(Duration::new(0, 1_000_000_000u32 / 60));
        //  We set fulfill our window with red.
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        // We draw it.
        canvas.clear();

        // The rectangle switch happens here:
        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                // In case of error, we do nothing...
                true
            }
        };
        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };

        // Copy our texture into the window.
        canvas
            .copy(
                square_texture,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE), // We copy it at the top-left of the window with a 32x32 size.
            )
            .expect("Couldn't copy texture into window");
        // We update window's display.
        canvas.present(); // update the windows' display.

        // We sleep enough to get ~60 fps. If we don't call this, the program will take
        // 100% of a CPU time.
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
