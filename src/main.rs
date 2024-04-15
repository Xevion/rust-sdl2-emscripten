use std::process;
use std::time::Duration;

use sdl2::rwops::RWops;
use sdl2::ttf;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};


static FONT_DATA: &[u8] = include_bytes!("../assets/TerminalVector.ttf");
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

#[cfg(not(target_os = "emscripten"))]
fn ttf_context() -> ttf::Sdl2TtfContext {
    ttf::init().unwrap()
}

// Honestly, I don't know why this is necessary. I'm just copying from https://github.com/aelred/tetris/blob/0ad88153db1ca7962b42277504c0f7f9f3c675a9/tetris-sdl/src/main.rs#L88-L92
#[cfg(target_os = "emscripten")]
fn ttf_context() -> &'static ttf::Sdl2TtfContext {
    // Deliberately leak so we get a static lifetime
    Box::leak(Box::new(ttf::init().unwrap()))
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

    let ttf_ctx = ttf_context();

    let mut prev = now();

    let font_data = RWops::from_bytes(FONT_DATA).unwrap();
    let font_size = 32;
    let font = ttf_ctx.load_font_from_rwops(font_data, font_size).unwrap();

    let fruit_atlas = texture_creator
        .load_texture("./assets/fruit.png")
        .expect("could not load texture");

    let target_fps = 60;
    let frame_time = Duration::from_secs_f32(1.0) / target_fps;
    let mut frame = 0;

    let mut n = 0;
    let mut avg_fps = 0f64;

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

        // draw fps counter
        let surface = font
            .render(&format!("{:.2}", avg_fps))
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let _ = canvas.copy(&texture, None, Rect::new(0, 0, 100, 100));

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
        } else {
            let excess = elapsed - frame_time;
            println!("! excess: {:?} ({:?})", excess, t2);
        }

        let duration = Duration::from_secs_f64(now() - prev);
        let fps = 1f64 / (duration.as_secs_f64());
        prev = now();
        
        n += 1;
        let a = 1f64 / n as f64;
        let b = 1f64 - a;
        avg_fps = a * fps + b * avg_fps;
    }
}
