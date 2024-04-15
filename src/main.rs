use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use rand::seq::IteratorRandom;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use web_time::{Duration, Instant};

static BLACK: Color = Color::RGB(0, 0, 0);

#[cfg(target_os = "emscripten")]
mod emscripten;

#[cfg(not(target_os = "emscripten"))]
fn sleep(ms: u32) {
    std::thread::sleep(Duration::from_millis(ms as u64));
}

#[cfg(target_os = "emscripten")]
fn now() -> f64  {
    emscripten::emscripten::now() / 1000f64
}

#[cfg(not(target_os = "emscripten"))]
fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

#[cfg(target_os = "emscripten")]
fn sleep(ms: u32) {
    emscripten::emscripten::sleep(ms);
}

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

    let mut canvas = match window.into_canvas().accelerated().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err),
    };

    let texture_creator = canvas.texture_creator();
    let mut point = Point::new(0, 0);

    // let ctx = Rc::new(RefCell::new(ctx));
    // let canvas = Rc::new(RefCell::new(canvas));
    // let texture_creator = Rc::new(texture_creator);

    let fruit_atlas = texture_creator
        .load_texture("./assets/fruit.png")
        .expect("could not load texture");

    let target_fps = 60;
    let frame_time = Duration::from_secs_f32(1.0) / target_fps;
    println!("Frame time: {:?}", frame_time);
    let mut frame = 0;

    loop {
        let start = now();
        let mut moved = false;
        for event in ctx.event_pump().unwrap().poll_iter() {
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

                        // random number between 2 and 6
                        let sleep_time = (15..=500).choose(&mut rand::thread_rng()).unwrap();
                        println!("Sleeping for {} ms", sleep_time);
                        sleep(sleep_time);
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
            let canvas_size = canvas.window().size();
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

        let t2 = now();
        let elapsed = Duration::from_secs_f64(t2 - start);
        if elapsed < frame_time {
            let sleep_time = frame_time - elapsed;
            sleep(sleep_time.as_millis() as u32);
            // println!("elapsed: {:?} sleep: {:?}ms ({:?})", elapsed, sleep_time, t2);
        } else {
            let excess = elapsed - frame_time;
            println!("! excess: {:?} ({:?})", excess, t2);
        }

        let t3 = now();
        let duration = Duration::from_secs_f64(t3 - start);
        let fps = 1f64 / (duration.as_secs_f64());
        println!("FPS: {}", fps);
    }
}
