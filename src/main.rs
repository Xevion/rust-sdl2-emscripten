use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

static BLACK: Color = Color::RGB(0, 0, 0);

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("Hello, Rust / SDL2 / WASM!", 640, 480)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err),
    };

    let canvas = match window.into_canvas().accelerated().present_vsync().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err),
    };

    let texture_creator = canvas.texture_creator();
    let mut point = Point::new(0, 0);

    let ctx = Rc::new(RefCell::new(ctx));
    let canvas = Rc::new(RefCell::new(canvas));
    let texture_creator = Rc::new(texture_creator);

    let fruit_atlas = texture_creator
        .load_texture("./assets/fruit.png")
        .expect("could not load texture");

    let mut frame = 0;

    loop {
        let mut moved = false;
        for event in ctx.borrow_mut().event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    process::exit(1);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Space => {
                        frame += 1;
                        if frame > 7 {
                            frame = 0;
                        }
                    }
                    Keycode::Left => {
                        point.x -= 32;
                        moved = true;
                    }
                    Keycode::Right => {
                        point.x += 32;
                        moved = true;
                    }
                    Keycode::Up => {
                        point.y -= 32;
                        moved = true;
                    }
                    Keycode::Down => {
                        point.y += 32;
                        moved = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Handle wrapping at the edges
        if moved {
            let canvas_size = canvas.borrow().window().size();
            if point.x < 0 {
                point.x = canvas_size.0 as i32 - 32;
            } else if point.x >= 640 {
                point.x = 0;
            }
            if point.y < 0 {
                point.y = canvas_size.1 as i32 - 32;
            } else if point.y >= canvas_size.1 as i32 {
                point.y = 0;
            }
        }

        let mut canvas = canvas.borrow_mut();

        canvas.set_draw_color(BLACK);
        canvas.clear();

        canvas
            .copy_ex(
                &fruit_atlas,
                Rect::new(32 * frame, 0, 32, 32),
                Rect::new(point.x, point.y, 32, 32),
                0.0,
                Some(Point::new(0, 0)),
                false,
                false,
            )
            .expect("could not draw texture");

        canvas.present();
    }

    // #[cfg(target_family = "wasm")]
    // {
    //     use crate::emscripten::emscripten::set_main_loop_callback;
    //     set_main_loop_callback(main_loop);
    // }

    // #[cfg(not(target_family = "wasm"))]
    // {
    //     use std::thread::sleep;
    //     use std::time::Duration;
    //     loop {
    //         main_loop();
    //         sleep(Duration::from_millis(10))
    //     }
    // }
}
