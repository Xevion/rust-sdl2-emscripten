use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

#[cfg(target_family = "wasm")]
pub mod emscripten;

static BLACK: Color = Color::RGB(0, 0, 0);
static WHITE: Color = Color::RGB(255, 255, 255);

pub fn main_loop(ctx: Rc<RefCell<Sdl>>, rect: Rc<RefCell<Rect>>, canvas: Rc<RefCell<WindowCanvas>>) -> impl FnMut() {
    let mut events = ctx.borrow_mut().event_pump().unwrap();

    move || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    rect.borrow_mut().x -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rect.borrow_mut().x += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    rect.borrow_mut().y -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    rect.borrow_mut().y += 10;
                },
                _ => {}
            }
        }

        let _ = canvas.borrow_mut().set_draw_color(BLACK);
        let _ = canvas.borrow_mut().clear();
        let _ = canvas.borrow_mut().set_draw_color(WHITE);
        let _ = canvas.borrow_mut().fill_rect(rect.borrow().clone());
        let _ = canvas.borrow_mut().present();
    }

}