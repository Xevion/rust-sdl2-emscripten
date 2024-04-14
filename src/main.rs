use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

static BLACK: Color = Color::RGB(0, 0, 0);
static WHITE: Color = Color::RGB(255, 255, 255);

//     export EMCC_CFLAGS="-s USE_SDL=2"
//     cargo build --target asmjs-unknown-emscripten && open index.html

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

    let canvas = match window.into_canvas().present_vsync().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err),
    };

    let rect = Rect::new(0, 0, 10, 10);

    let ctx = Rc::new(RefCell::new(ctx));
    let rect = Rc::new(RefCell::new(rect));
    let canvas = Rc::new(RefCell::new(canvas));

    let mut events = ctx.borrow_mut().event_pump().unwrap();

    let mut main_loop = move || {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    process::exit(1);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    rect.borrow_mut().x -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    rect.borrow_mut().x += 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    rect.borrow_mut().y -= 10;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    rect.borrow_mut().y += 10;
                }
                _ => {}
            }
        }

        let _ = canvas.borrow_mut().set_draw_color(BLACK);
        let _ = canvas.borrow_mut().clear();
        let _ = canvas.borrow_mut().set_draw_color(WHITE);
        let _ = canvas.borrow_mut().fill_rect(rect.borrow().clone());
        let _ = canvas.borrow_mut().present();
    };

    #[cfg(target_family = "wasm")]
    use pacman::emscripten;

    #[cfg(target_family = "wasm")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_family = "wasm"))]
    {
        use std::thread::sleep;
        use std::time::Duration;
        loop {
            main_loop();
            sleep(Duration::from_millis(10))
        }
    }
}
