// #![windows_subsystem = "windows"]
use std::process;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mixer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::rwops::RWops;
use sdl2::ttf;

// static FONT_DATA: &[u8] = include_bytes!("../assets/TerminalVector.ttf");
static MUSIC_DATA: &[u8] = include_bytes!("../assets/tetris.ogg");
static BLACK: Color = Color::RGB(0, 0, 0);

#[cfg(target_os = "emscripten")]
mod emscripten;

mod store;

#[cfg(not(target_os = "emscripten"))]
fn sleep(ms: u32) {
    std::thread::sleep(Duration::from_millis(ms as u64));
}

#[cfg(target_os = "emscripten")]
fn now() -> f64 {
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

#[cfg(not(target_os = "windows"))]
fn hide_console_window() {}

#[cfg(target_os = "windows")]
fn hide_console_window() {
    if !atty::is(atty::Stream::Stdout) {
        unsafe { winapi::um::wincon::FreeConsole() };
    }
}

fn main() {
    hide_console_window();
    
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    
    let window = match video_ctx
    .window("rust-sdl2-emscripten", 640, 480)
    .position_centered()
    .resizable()
    .allow_highdpi()
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

    // canvas.set_logical_size(1000, 1000);

    let texture_creator = canvas.texture_creator();
    let mut point = Point::new(0, 0);

    let ttf_ctx = ttf_context();

    mixer::open_audio(
        44_100,
        sdl2::mixer::AUDIO_S16LSB,
        sdl2::mixer::DEFAULT_CHANNELS,
        1024,
    )
    .unwrap();

    let storage = store::Store {};

    let mut volume = storage.volume().map_or_else(
        || {
            println!("No volume stored, using default");
            1
        },
        |v| v,
    );
    print!("Volume: {}", volume);
    mixer::Music::set_volume(volume as i32);

    let music_data = RWops::from_bytes(MUSIC_DATA).unwrap();
    let music = mixer::LoaderRWops::load_music(&music_data).unwrap();
    music.play(-1).unwrap();

    let mut prev = now();

    // let font_data = RWops::from_bytes(FONT_DATA).unwrap();
    // let font_size = 12;
    // let font = ttf_ctx.load_font_from_rwops(font_data, font_size).unwrap();

    let font = ttf_ctx
        .load_font("./assets/TerminalVector.ttf", 12)
        .unwrap();

    let fruit_atlas = texture_creator
        .load_texture("./assets/fruit.png")
        .expect("could not load texture");

    let target_fps = 60;
    let frame_time = Duration::from_secs_f32(1.0) / target_fps;
    let mut frame = 0;

    let mut n = 0;
    let mut avg_fps = 0f64;

    let mut mouse = Point::new(0, 0);
    let mut previous_focus = false;

    loop {
        let start = now();
        let mut moved = false;

        for event in ctx.event_pump().unwrap().poll_iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    mouse = Point::new(x, y);
                }
                Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::Resized(w, h) => {
                        println!("Resized to {}x{}", w, h);
                        canvas.window_mut().set_size(w as u32, h as u32).unwrap();
                    }
                    _ => {}
                },
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
                        if ctx
                            .keyboard()
                            .mod_state()
                            .contains(sdl2::keyboard::Mod::LSHIFTMOD)
                        {
                            if volume < 128 {
                                volume += 1;
                                mixer::Music::set_volume(volume as i32);
                                storage.set_volume(volume);
                            }
                        } else {
                            point.y -= 32;
                            moved = true;
                        }
                    }
                    Keycode::Down => {
                        // Shift means modify volume
                        if ctx
                            .keyboard()
                            .mod_state()
                            .contains(sdl2::keyboard::Mod::LSHIFTMOD)
                        {
                            if volume > 0 {
                                volume -= 1;
                                mixer::Music::set_volume(volume as i32);
                                storage.set_volume(volume);
                            }
                        } else {
                            point.y += 32;
                            moved = true;
                        }
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

        let focused = ctx.mouse().focused_window_id().is_some();
        if focused != previous_focus {
            println!("Focus: {:?}", focused);
            previous_focus = focused;
        }

        // Draw a 32x32 square at the mouse position
        if focused {
            let mouse_x = (mouse.x / 32 * 32) as i16;
            let mouse_y = (mouse.y / 32 * 32) as i16;
            let color = Color::RGB(255, 255, 255);

            let _ = canvas.line(mouse_x, mouse_y, mouse_x + 32, mouse_y, color);
            let _ = canvas.line(mouse_x + 32, mouse_y, mouse_x + 32, mouse_y + 32, color);
            let _ = canvas.line(mouse_x + 32, mouse_y + 32, mouse_x, mouse_y + 32, color);
            let _ = canvas.line(mouse_x, mouse_y + 32, mouse_x, mouse_y, color);
        }
        // canvas.line(top_left.x as i16, top_left.y as i16, top_left.x as i16 + 32, top_left.y as i16 + 32, color);

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
            .unwrap();

        // draw fps counter
        let text = format!("{:.0}", avg_fps);
        let surface = font
            .render(&text)
            .blended(Color::RGBA(255, 255, 255, 50))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let _ = canvas.copy(
            &texture,
            None,
            Rect::new(
                640i32 - (25i32 * text.len() as i32),
                0,
                25 * text.len() as u32,
                40,
            ),
        );

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
